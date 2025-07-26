use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::e_commerce::ProductStatus;




#[derive(Debug, Deserialize)]
pub struct BundleItem{
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateBundle{
    pub name: String,
    pub price: Decimal,
    #[serde(default)]
    pub status: ProductStatus,
    pub items: Vec<BundleItem>,

    #[serde(skip_deserializing)]
    pub created_by: Option<Uuid>
}

impl CreateBundle {

    pub fn record(mut self, who: Uuid) -> Self{
        self.created_by = Some(who);
        self
    }
}




#[derive(Debug, Deserialize)]
pub struct EditBundle{
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stocks: Option<Decimal>,
    pub remove_product: Option<Vec<i32>>,
    pub add_product: Option<Vec<BundleItem>>,
    pub edit_product: Option<Vec<BundleItem>>
}

impl EditBundle {

    pub fn bundle_has_some(&self) -> bool {
        self.name.is_some() |
        self.description.is_some() |
        self.price.is_some() |
        self.stocks.is_some()  
    } 

    pub fn bundle_item_has_some(&self) -> bool {
        self.remove_product.is_some() |
        self.add_product.is_some() |
        self.edit_product.is_some()
    }
}



#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct JoinedProduct{
    pub quantity: i32,
    pub product_id: i32,
    pub product_name: String,
} 

#[derive(Debug, Serialize, FromRow)]
pub struct BundleJoinedProduct{
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub status: ProductStatus,

    #[sqlx(skip)]
    pub items: Option<Vec<JoinedProduct>>
} 

impl BundleJoinedProduct {

    pub fn join_product(
        &mut self,
        items: Vec<JoinedProduct>
    ){
        self.items = Some(items);
    } 
}


