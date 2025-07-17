use actix_web::{guard, web, HttpResponse, Responder};
use middleware::auth_check::{AdminValidator, EmployeeValidator, UserValidator};
use serde_json::{Value, json};
use types::{app_state::AppState, INTERNAT_ERR_MSG};

use crate::SECRET_KEY;

mod admin;
mod auth;
mod bundle;
mod product;
mod service;

mod util;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg

        // Public routes
        .service(
            web::scope("")
                .configure(auth::public::scope)
        )




        // User Authenticated routes
        .service(
            web::scope("")
                .wrap(UserValidator::new(&SECRET_KEY))
                .configure(auth::user::scope)
                .service(
                    web::scope("/product")
                    .configure(product::user::scope)
                )


                
        )



        // Employees Authenticated routes
        .service(
            web::scope("")
                .wrap(EmployeeValidator::new(&SECRET_KEY))
                .service(
                    web::scope("/product")
                    .configure(product::employee::scope)
                )
        )



        // Admin Authenticated routes
        .service(
            web::scope("")
                .wrap(AdminValidator::new(&SECRET_KEY))
                .service(
                    web::scope("/product")
                    .configure(product::admin::scope)
                )
        )


        // Reroute to invalide route to this service
        .default_service(web::to(invalid_url))
    
    ;
        
}






fn internal_err_json<T: AsRef<str>>(err_msg: T) -> Value {
    let msg = err_msg.as_ref();

    json!({
        "status": "error",
        "message": INTERNAT_ERR_MSG.to_owned() + msg
    })
}

async fn invalid_url() -> impl Responder {
    HttpResponse::ServiceUnavailable().json(json!({
        "error": "service invalid"
    }))
}






// Template use 
// each module must contain
mod employee_scope{
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {

    }
}






