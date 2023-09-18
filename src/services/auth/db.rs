use actix_web::dev::ServiceRequest;
use sqlx::PgPool;

use crate::models::User;

pub async fn get_current_user(pool: &PgPool, req: &ServiceRequest) -> Option<User>
{
    None
}
