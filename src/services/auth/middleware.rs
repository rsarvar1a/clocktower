use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::LOCATION,
    Error as ActixError, HttpResponse,
};
use serde::{Deserialize, Serialize};

use super::db;
use crate::utils;

use std::{
    cell::RefCell,
    future::{self, Future, Ready},
    pin::Pin,
    rc::Rc,
    task::Poll,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum AuthPolicyAction
{
    Unauthorized,
    HTTPRedirect(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AuthPolicyCondition
{
    Disallow,
    Require,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthPolicy
{
    pub action: AuthPolicyAction,
    pub condition: AuthPolicyCondition,
}

impl Default for AuthPolicy
{
    fn default() -> Self
    {
        AuthPolicy {
            action: AuthPolicyAction::Unauthorized,
            condition: AuthPolicyCondition::Require,
        }
    }
}

impl AuthPolicy
{
    pub fn disallow(&self) -> AuthPolicy
    {
        AuthPolicy {
            action: self.action.clone(),
            condition: AuthPolicyCondition::Disallow,
        }
    }

    pub async fn implement<S>(
        &self,
        service: S,
        req: ServiceRequest,
        logged_in: bool,
    ) -> Result<ServiceResponse, ActixError>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = ActixError> + 'static,
        S::Future: 'static,
    {
        match self.condition
        {
            AuthPolicyCondition::Require => match logged_in
            {
                true => service.call(req).await.map(|res| res.map_into_boxed_body()),
                false => match &self.action
                {
                    AuthPolicyAction::Unauthorized => Ok(req.into_response(HttpResponse::Unauthorized().finish())),
                    AuthPolicyAction::HTTPRedirect(s) =>
                    {
                        Ok(req.into_response(HttpResponse::Found().append_header((LOCATION, s.as_str())).finish()))
                    }
                },
            },
            AuthPolicyCondition::Disallow => match logged_in
            {
                false => service.call(req).await.map(|res| res.map_into_boxed_body()),
                true => match &self.action
                {
                    AuthPolicyAction::Unauthorized => Ok(req.into_response(HttpResponse::Unauthorized().finish())),
                    AuthPolicyAction::HTTPRedirect(s) =>
                    {
                        Ok(req.into_response(HttpResponse::Found().append_header((LOCATION, s.as_str())).finish()))
                    }
                },
            },
        }
    }

    pub fn redirect(&self, url: &str) -> AuthPolicy
    {
        AuthPolicy {
            action: AuthPolicyAction::HTTPRedirect(url.to_owned()),
            condition: self.condition.clone(),
        }
    }

    pub fn require(&self) -> AuthPolicy
    {
        AuthPolicy {
            action: self.action.clone(),
            condition: AuthPolicyCondition::Require,
        }
    }

    pub fn unauthorized(&self) -> AuthPolicy
    {
        AuthPolicy {
            action: AuthPolicyAction::Unauthorized,
            condition: self.condition.clone(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthService
{
    pub policy: AuthPolicy,
}

impl AuthService
{
    pub fn new(policy: AuthPolicy) -> AuthService
    {
        AuthService { policy }
    }
}

impl<S> Transform<S, ServiceRequest> for AuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Error = ActixError;
    type Response = ServiceResponse<BoxBody>;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future
    {
        future::ready(Ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
            policy: self.policy.clone(),
        }))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthMiddleware<S>
{
    pub service: Rc<RefCell<S>>,
    pub policy: AuthPolicy,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Error = ActixError;
    type Response = ServiceResponse<BoxBody>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>>
    {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future
    {
        let service = self.service.clone();
        let policy = self.policy.clone();

        Box::pin(async move {
            let pool = utils::pool(&req);
            let current_user = db::get_current_user(&pool, &req).await;
            policy.implement(service, req, current_user.is_some()).await
        })
    }
}
