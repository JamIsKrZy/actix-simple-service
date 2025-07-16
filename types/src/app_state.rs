use std::sync::Arc;

use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{
        SaltString,
        rand_core::{self, OsRng},
    },
};
use base64::{Engine as _, engine::general_purpose};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::jwt_ess::Claim;

#[derive(Debug)]
pub enum AppStateErr {
    FailedConnectionDB,
    SecretKeyUndefined,
    FailedToken,
    ArgonInitFailure,
    HashFail,
}

pub struct JWTEss {
    pub header: Header,
    pub enc_key: EncodingKey,
}

#[derive()]
pub struct AppState {
    pub jwt: JWTEss,
    pub db: Pool<Postgres>,
    pub hashifier: Argon2<'static>,
}

pub trait HashifyPassword {
    type E;

    fn hashify_pass_with_salt(&self, password: String) -> Result<(String, String), Self::E>;

    fn hashify_pass<T: AsRef<str>>(&self, password: T, salt: T) -> Result<String, Self::E>;
}

impl JWTEss {
    fn default<T: AsRef<[u8]>>(secret: T) -> Result<Self, AppStateErr> {
        Ok(Self {
            header: Header::default(),
            enc_key: EncodingKey::from_secret(secret.as_ref()),
        })
    }
}

impl AppState {
    async fn init_db_pool() -> Result<Pool<Postgres>, AppStateErr> {
        let url = dotenvy::var("DATABASE_URL").map_err(|_| AppStateErr::FailedConnectionDB)?;

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .map_err(|_| AppStateErr::FailedConnectionDB)?;

        Ok(pool)
    }

    pub async fn init(secret: &str) -> Result<Self, AppStateErr> {

        let jwt = JWTEss::default(&secret)?;
        let db = Self::init_db_pool().await?;

        let hashifier = Argon2::new(Algorithm::Argon2d, Version::V0x13, Params::DEFAULT);

        Ok(Self {
            hashifier,
            jwt,
            db,
        })
    }

    pub fn encode_claim(&self, claim: &Claim) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&self.jwt.header, claim, &self.jwt.enc_key)
    }

}

impl HashifyPassword for AppState {
    type E = AppStateErr;

    fn hashify_pass_with_salt(&self, password: String) -> Result<(String, String), Self::E> {
        let salt = SaltString::generate(&mut OsRng).to_string();
        let mut hashed = vec![0u8; 161];

        self.hashifier
            .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut hashed)
            .map_err(|_| AppStateErr::HashFail)?;

        Ok((general_purpose::STANDARD.encode(hashed), salt))
    }

    fn hashify_pass<T: AsRef<str>>(&self, password: T, salt: T) -> Result<String, Self::E> {
        let mut hashed = vec![0u8; 161];

        self.hashifier
            .hash_password_into(
                password.as_ref().as_bytes(),
                salt.as_ref().as_bytes(),
                &mut hashed,
            )
            .map_err(|_| AppStateErr::HashFail)?;

        Ok(general_purpose::STANDARD.encode(hashed))
    }
}
