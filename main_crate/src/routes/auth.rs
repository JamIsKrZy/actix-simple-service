use actix_web::{get, web, HttpResponse, Responder};
use types::{schemas::json::SignUpUser, AppState};



pub fn scope(cfg: &mut web::ServiceConfig){
    cfg.service(
    web::scope("/verify")
            .service(verify_admin)
            .service(check_permission)
    )
    .service(login)
    .service(sign_up)
    ;
}


#[get("/validate-admin")]
async fn verify_admin() -> impl Responder{
    HttpResponse::NotImplemented()
}

#[get("/check_permissions")]
async fn check_permission() -> impl Responder{
    HttpResponse::NotImplemented()
}

#[get("/login")]
async fn login() -> impl Responder{
    HttpResponse::NotImplemented()
}

#[get("/sign-up")]
async fn sign_up(
    info: web::Json<SignUpUser>,
    data: web::Data<AppState>
) -> impl Responder{

    
    HttpResponse::NotImplemented()
}


