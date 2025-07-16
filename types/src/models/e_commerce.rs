use chrono::NaiveDateTime;
use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "purchase_type", rename_all = "lowercase")]
pub enum PurchaseType{
    Product,
    Bundle
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "product_status", rename_all = "lowercase")]
pub enum ProductStatus{
    Available,
    Unavailable
}




#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(unused_attributes)]
pub struct Cart{
    user_id: Uuid,
    item_type: PurchaseType,
    item_id: i32,
    quantity: i16,
    time_created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(unused_attributes)]
pub struct Product{
    id: i32,
    product_name: String,
    description: String,
    price: Decimal,
    stocks: i64,

    created_by: Uuid,
    edited_by: Uuid,
    edited_time: NaiveDateTime,
    status: ProductStatus,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(unused_attributes)]
pub struct Bundle{
    id: i32,
    product_name: String,
    price: Decimal,

    status: ProductStatus,
    created_by: Uuid,
    edited_by: Uuid,
    edited_time: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(unused_attributes)]
pub struct BundleItem{
    bundle_id: i32,
    product_id: i32,
    quantity: i32,
}
