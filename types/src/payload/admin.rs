use serde::{Deserialize, Serialize};

use crate::{app_state::HashifyPassword, models::init::{NewSaltUser, UserRole}};


#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser{
    pub email: String,
    pub password: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub location: Option<String>,
    pub role: UserRole
}


impl NewSaltUser for NewUser{
    type E = ();
    type HashedOutput = NewHashedUser;

    fn generate_hash_salt<T: HashifyPassword>(
        self, 
        hasher: &T
    ) -> Result<Self::HashedOutput, Self::E> {
        
        let (password, salt) = hasher.hashify_pass_with_salt(self.password)
            .map_err(|_| ())?;

        Ok(NewHashedUser { 
            email: self.email, 
            username: self.username, 
            password , 
            salt,
            first_name: self.first_name, 
            last_name: self.last_name, 
            phone_no: self.phone_no, 
            location: self.location, 
            role: self.role 
        })

    }
    
}



#[derive(Debug, Deserialize, Serialize)]
pub struct NewHashedUser{
    pub email: String,
    pub password: String,
    pub salt: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub location: Option<String>,
    pub role: UserRole
}


#[derive(Debug, Deserialize)]
pub struct EditUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_no: Option<String>,
    pub location: Option<String>,
    pub role: Option<String>
}