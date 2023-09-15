use crate::models::Email;
use crate::services::auth::method::Method;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct User
{
    pub id: Uuid,
    pub username: String,
    pub method: Method,
    pub email: Option<Email>,
}
