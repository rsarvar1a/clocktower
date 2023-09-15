use crate::services;

use actix_web::web::{scope, ServiceConfig};

pub fn routes(ctx: &mut ServiceConfig) -> ()
{
    ctx.service(scope("/auth").configure(services::auth::routes));
}
