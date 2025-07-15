use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, Value};
use types::INTERNAT_ERR_MSG;

mod auth;
mod home;
mod admin;


fn internal_err_json<T: AsRef<str>>(err_msg: T) -> Value{
    let msg = err_msg.as_ref();

    json!({
        "status": "error",
        "message": INTERNAT_ERR_MSG.to_owned() + msg  
    })
}


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