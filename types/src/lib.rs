use chrono::{TimeDelta, TimeZone, Utc};
use serde::{Deserialize, Serialize};



mod utils;
pub mod schemas;
pub mod models;
pub mod app_state;




pub const INTERNAT_ERR_MSG: &'static str = "Something went wrong: ";
const EXPECTED_EXPIRATION: i64 = 24;



#[derive(Debug, Serialize, Deserialize)]
pub struct Claim{
    sub: String,
    exp: usize
}


impl Claim {

    pub fn new(sub: String) -> Option<Self> {

        let exp = Utc::now()
            .checked_add_signed(TimeDelta::hours(EXPECTED_EXPIRATION))?
            .timestamp() as usize;

        Some(Claim { 
            sub, 
            exp 
        })
    } 

}

