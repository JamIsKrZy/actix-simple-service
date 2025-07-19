use derive_more::Display;


pub mod user;
pub mod product;


#[derive(Debug, Display)]
pub enum QueryErr{
    FailedToInsert(String),
    FailedToCollect(String),
    FailedToPatch(String),
    EmptyValue
}