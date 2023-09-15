use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Provider
{
    Discord,
    Google,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Method
{
    Password(String),
    OAuth(Provider, String),
}
