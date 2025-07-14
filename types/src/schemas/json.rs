use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, )]
pub struct SignUpUser{
    email: String,
    password: String,

    username: String,
    first_name: String,
    last_name: String,
    phone_no: String,

    location: Option<String>,
}