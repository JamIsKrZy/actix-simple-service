use derive_more::Display;


pub mod user;
pub mod product;
pub mod bundle;


#[derive(Debug, Display)]
pub enum QueryErr{
    EmptyItem,
    ItemNoHistoryRecord,

    FailedToBeginTransaction,
    FailedCommitTransaction,

    FailedToInsert(String),
    FailedToCollect(String),
    FailedToPatch(String),
    FailedToJoin
}