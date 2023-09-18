mod db;
mod oauth;
mod routes;

pub mod method;
pub mod middleware;

use actix_web::web::{self, scope, ServiceConfig};

use self::middleware::{AuthPolicy, AuthService};

pub fn routes(ctx: &mut ServiceConfig) -> ()
{
    ctx.service(
        scope("/auth")
            .wrap(AuthService::new(AuthPolicy::default().disallow()))
            .route("/login", web::post().to(routes::login))
            .route("/register", web::post().to(routes::register)),
    );
}
