use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
