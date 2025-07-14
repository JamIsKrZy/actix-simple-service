use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct ListAmount{
    pub offset: Option<usize>,
    pub max: Option<usize>
}