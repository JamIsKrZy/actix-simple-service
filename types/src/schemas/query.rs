use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListAmount {
    pub offset: Option<i64>,
    pub max: Option<i64>,
}
