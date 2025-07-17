use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;





#[derive(Debug, Deserialize)]
pub struct ItemQuery{
    pub id: i32
}


#[derive(Debug, Deserialize)]
pub struct CreateProduct{
    name: String,
    description: Option<String>,
    price: Decimal,
    stock: i64
}

impl CreateProduct {

    pub fn set_record(self, user_id: &Uuid) -> IntoProduct {
        IntoProduct{
            created_by: user_id,
            name: self.name,
            description: self.description,
            price: self.price,
            stock: self.stock,
        }
    } 

}


pub struct IntoProduct<'a>{
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i64,
    pub created_by: &'a Uuid
}