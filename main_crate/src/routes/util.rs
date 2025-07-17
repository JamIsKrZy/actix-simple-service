use actix_web::{HttpMessage, HttpRequest, HttpResponse, ResponseError};
use serde_json::json;
use types::jwt_ess::Claim;


pub enum Error{
    ClaimNotFound,
    
}







pub fn claim_ref(req: &HttpRequest) -> Result<Claim, Error> {
    req.extensions_mut()
    .get::<Claim>()
    .cloned()
    .ok_or(Error::ClaimNotFound)
}