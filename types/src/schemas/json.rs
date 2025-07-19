use serde::{Deserialize, Serialize};

use crate::models::init::{RegisterUser, NewSaltUser, USER_EMAIL_CAP, User, UserRole};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignUpUser {
    email: String,
    pub password: String,

    username: String,
    first_name: String,
    last_name: String,
    phone_no: String,

    location: Option<String>,
}

impl NewSaltUser for SignUpUser {
    type E = ();
    type HashedOutput = RegisterUser;

    fn generate_hash_salt<T: crate::app_state::HashifyPassword>(
        self,
        hasher: &T,
    ) -> Result<Self::HashedOutput, Self::E> {
        let (hash_pass, salt) = hasher
            .hashify_pass_with_salt(self.password)
            .map_err(|_| ())?;

        Ok(RegisterUser {
            email: self.email,
            password: hash_pass,
            salt: salt,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            location: self.location,
            phone_no: self.phone_no,
        })
    }
    
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl LoginUser {
    pub fn check_len(&self) -> Result<(), ()> {
        if self.username.len() > USER_EMAIL_CAP {
            return Err(());
        }
        Ok(())
    }
}
