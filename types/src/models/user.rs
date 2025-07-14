

use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name="user_role", rename_all="lowercase")]
pub enum UserRole{
    Admin,
    Regular,
    Staff
}

#[allow(dead_code)]
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User{
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: String,

    pub username: String,
    #[serde(rename="firstName")]
    pub first_name: String,
    #[serde(rename="lastName")]
    pub last_name: String,

    pub location: Option<String>,
    #[serde(rename="phoneNo")]
    pub phone_no: String,

    pub role: UserRole
}