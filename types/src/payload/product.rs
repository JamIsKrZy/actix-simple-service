use chrono::{DateTime, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use crate::models::e_commerce::ProductStatus;



#[derive(Debug, Deserialize)]
pub struct ItemQuery{
    pub id: i32
}


#[derive(Debug, Deserialize)]
pub struct CreateProduct{
    name: String,
    description: Option<String>,
    price: Decimal,
    stocks: i64
}

impl CreateProduct {

    pub fn set_record(self, user_id: &Uuid) -> TheProduct {
        TheProduct{
            created_by: user_id,
            name: self.name,
            description: self.description,
            price: self.price,
            stocks: self.stocks,
        }
    } 

}


pub struct TheProduct<'a>{
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stocks: i64,
    pub created_by: &'a Uuid
}



#[derive(Debug, Deserialize)]
pub struct EditProduct{
    name: Option<String>,
    description: Option<String>,
    price: Option<Decimal>,
    stocks: Option<i64>,
    status: Option<ProductStatus>
}

impl EditProduct {

    pub fn record_by(self, who: Uuid) -> RecordedEditProduct{

        RecordedEditProduct { 
            name: self.name, 
            description: self.description, 
            price: self.price, 
            stocks: self.stocks, 
            edited_by: who,
            edited_time: Utc::now(), 
            status: self.status
        }
    }
}

/// Used for patching product with 
/// the client edited record
pub struct RecordedEditProduct{
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stocks: Option<i64>,
    pub edited_by: Uuid,
    pub edited_time: DateTime<Utc>,
    pub status: Option<ProductStatus>
}




#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ProductJoinedUser{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stocks: i64,

    
    pub created_by_id: Uuid,
    pub created_by_name: String,

    pub edited_by_id: Option<Uuid>,
    pub edited_by_name: Option<String>,

    pub edited_time: Option<NaiveDateTime>,
    pub status: ProductStatus,
}
