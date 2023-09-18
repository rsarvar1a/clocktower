use actix_web::{dev::ServiceRequest, web::Data};
use sqlx::PgPool;

pub fn pool(req: &ServiceRequest) -> PgPool
{
    req.app_data::<Data<PgPool>>().unwrap().get_ref().clone()
}
