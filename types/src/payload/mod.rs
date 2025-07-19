use serde::Deserialize;

pub mod admin;
pub mod product;


#[derive(Debug, Deserialize)]
pub struct QueryBounds {
    pub offset: Option<i64>,
    pub max: Option<i64>,
}

impl QueryBounds {
    pub fn finalize(self) -> (i64, i64){
        (
            self.max.unwrap_or(25),
            self.offset.unwrap_or(0).saturating_sub(1),
        )
    } 
}
