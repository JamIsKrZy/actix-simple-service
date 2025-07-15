
use std::sync::Arc;

use argon2::{password_hash::{rand_core::{self, OsRng}, SaltString}, Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose, Engine as _};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};

use crate::Claim;



#[derive(Debug)]
pub enum AppStateErr{
    FailedConnectionDB,
    SecretKeyUndefined,
    FailedToken,
    ArgonInitFailure,
    HashFail
}


pub struct JWTEss{
    pub header: Header,
    pub validator: Validation,
    pub enc_key: EncodingKey,
    pub dec_key: DecodingKey,
}

#[derive()]
pub struct AppState{
    secret: Arc<[u8]>,
    pub jwt: JWTEss,
    pub db: Pool<Postgres>,
    pub hashifier: Argon2<'static>
}


pub trait HashifyPassword{
    type E;

    fn hashify_pass_with_salt(
        &self, 
        password: String
    ) -> Result<(String, String), Self::E>; 

    fn hashify_pass<T: AsRef<str>>(
        &self, 
        password: T,
        salt: T,
    ) -> Result<String, Self::E>;    
}

impl JWTEss{

    fn default<T: AsRef<[u8]>>(secret: T) -> Result<Self, AppStateErr>{

        Ok( Self { 
            header: Header::default(), 
            validator: Validation::default(),
            enc_key: EncodingKey::from_secret(secret.as_ref()),
            dec_key: DecodingKey::from_secret(secret.as_ref())
        })
    }

}


impl AppState {

    async fn init_db_pool() -> Result<Pool<Postgres>, AppStateErr>{
        let url = dotenvy::var("DATABASE_URL")
            .map_err(|_| AppStateErr::FailedConnectionDB)?;


        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .map_err(|_| AppStateErr::FailedConnectionDB)?;

        Ok(pool)
    } 

    pub async fn init() -> Result<Self, AppStateErr>{

        let secret = dotenvy::var("SECRET_KEY")
            .map_err(|_| AppStateErr::SecretKeyUndefined)?
            .into_bytes()
            .into_boxed_slice();
        let secret = Arc::from(secret);    
        

        let jwt = JWTEss::default(&secret)?;
        let db = Self::init_db_pool().await?;

        let hashifier = Argon2::new(
                Algorithm::Argon2d, 
                Version::V0x13, 
                Params::DEFAULT
            ); 

        Ok(Self { 
            hashifier,
            secret,
            jwt,
            db
        })
    }

    pub fn encode_claim(&self, claim: &Claim) -> Result<String, jsonwebtoken::errors::Error>{
        encode(&self.jwt.header, claim, &self.jwt.enc_key)
    }


}

impl HashifyPassword for AppState{
    type E = AppStateErr;

    fn hashify_pass_with_salt(
        &self, 
        password: String
    ) -> Result<(String, String), Self::E> {



        let salt = SaltString::generate(&mut OsRng).to_string();
        let mut hashed = vec![0u8;161]; 

        self.hashifier.hash_password_into(
            password.as_bytes(), 
            salt.as_str().as_bytes(),
            &mut hashed
        ).map_err(|_| AppStateErr::HashFail)?;


    
        Ok((
            general_purpose::STANDARD.encode(hashed), 
            salt
        ))
    }
    
    fn hashify_pass<T: AsRef<str>>(
        &self, 
        password: T,
        salt: T,
    ) -> Result<String, Self::E> {
        
        let mut hashed = vec![0u8;161]; 

        self.hashifier.hash_password_into(
            password.as_ref().as_bytes(),
            salt.as_ref().as_bytes(),
            &mut hashed
        ).map_err(|_| AppStateErr::HashFail)?;


        Ok(general_purpose::STANDARD.encode(hashed))
    }
    
    
}



