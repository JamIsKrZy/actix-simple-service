pub mod app_state;
pub mod models;
pub mod schemas;
pub mod payload;
mod utils;

pub const INTERNAT_ERR_MSG: &'static str = "Something went wrong: ";
const EXPECTED_EXPIRATION: i64 = 24;

pub mod jwt_ess {

    use chrono::{TimeDelta, Utc};
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
    use serde::{de::value::UsizeDeserializer, Deserialize, Serialize};
    use uuid::Uuid;

    use crate::models::init::UserRole;

    pub fn encoding_key<T: AsRef<str>>(secret: T) -> (Header, EncodingKey) {
        (
            Header::default(),
            EncodingKey::from_secret(secret.as_ref().as_bytes()),
        )
    }

    pub fn decoding_key<T: AsRef<str>>(secret: T) -> (Validation, DecodingKey) {
        (
            Validation::default(),
            DecodingKey::from_secret(secret.as_ref().as_bytes()),
        )
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Claim {
        pub sub: Uuid,
        pub role: UserRole,
        iat: usize,
        exp: usize,
    }

    impl Claim {
        pub fn new(user_id: Uuid, role: UserRole) -> Option<Self> {
            let iat = Utc::now()
                .timestamp() as usize;

            let exp = Utc::now()
                .checked_add_signed(TimeDelta::hours(super::EXPECTED_EXPIRATION))?
                .timestamp() as usize;

            Some(Claim { sub: user_id, iat, exp, role })
        }
    }
}
