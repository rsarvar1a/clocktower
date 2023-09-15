use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Email
{
    address: String,
    verified: bool,
}
