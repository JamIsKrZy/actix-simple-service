use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::app_state::HashifyPassword;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Regular,
    Employee,
}

impl UserRole {


    pub fn is_admin(&self) -> bool {
        self == &UserRole::Admin
    }

    pub fn is_employee(&self) -> bool {
        self == &UserRole::Employee
    }

    pub fn is_user(&self) -> bool {
        self == &UserRole::Regular
    }
}

pub trait NewSaltUser {
    type E;
    type HashedOutput;

    fn generate_hash_salt<T: HashifyPassword>(self, hasher: &T) -> Result<Self::HashedOutput, Self::E>;
}

pub(crate) const USER_EMAIL_CAP: usize = 64;
pub(crate) const USER_USERNAME_CAP: usize = 16;
pub(crate) const USER_PASSWORD_CAP: usize = 255;

#[allow(dead_code)]
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: String,

    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,

    pub location: Option<String>,
    #[serde(rename = "phoneNo")]
    pub phone_no: String,

    pub role: UserRole,
}

/// This struct assumes that the user does not yet exist
/// and will be pushed to the table
///
#[derive(Debug, Serialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub salt: String,

    pub username: String,
    pub first_name: String,
    pub last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub phone_no: String,
}
