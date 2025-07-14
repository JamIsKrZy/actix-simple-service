
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};



pub mod schemas;
pub mod models;

#[derive(Debug)]
pub enum AppStateErr{
    FailedConnectionDB
}


pub const INTERNAT_ERR_MSG: &'static str = "Something went wrong: ";


#[derive(Debug)]
pub struct AppState{
    pub db: Pool<Postgres>
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

        let db_pool = Self::init_db_pool().await?;

        Ok(Self { 
            db:  db_pool
        })
    }
}