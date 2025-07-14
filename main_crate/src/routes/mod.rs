use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

mod auth;
mod home;
mod admin;


pub fn routes(cfg: &mut web::ServiceConfig){
    cfg
        .configure(auth::scope)
        .configure(home::scope)
        .configure(admin::scope)
        .default_service(web::to(invalid_url));
} 



async fn invalid_url() -> impl Responder {
    HttpResponse::ServiceUnavailable().json(json!({
        "error": "service invalid" 
    }))
}