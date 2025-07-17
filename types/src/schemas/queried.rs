use serde::Deserialize;
use uuid::Uuid;

use crate::models::init::UserRole;

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub struct UserLoginEssential {
    pub id: Uuid,
    pub salt: String,
    pub password: String,
    pub role: UserRole,
}
