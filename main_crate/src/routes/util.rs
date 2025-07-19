use actix_web::{HttpMessage, HttpRequest, HttpResponse, ResponseError};
use serde_json::json;
use types::jwt_ess::Claim;
use uuid::Uuid;

use crate::error::ServiceError;









pub fn get_claim(req: &HttpRequest) -> Result<Claim, ServiceError> {
    req.extensions_mut()
    .get::<Claim>()
    .cloned()
    .ok_or(ServiceError::CookieInternals)
}


pub fn get_claim_id(req: &HttpRequest) -> Result<Uuid, ServiceError> {
    req.extensions_mut()
    .get::<Claim>()
    .map(|claim| claim.sub.clone())
    .ok_or(ServiceError::CookieInternals)
}

