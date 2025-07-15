use actix_web::{cookie::{time::Duration, CookieBuilder}, get, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use types::{app_state::{AppState, HashifyPassword}, models::user::ToUser, schemas::{json::{LoginUser, SignUpUser}, queried::UserLoginEssential}, Claim};
use types::models::user::UserRole;
use crate::routes::internal_err_json;



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
async fn login(
    info: web::Json<LoginUser>,
    app: web::Data<AppState>
) -> impl Responder{
    
    if info.check_len().is_err() {
        return HttpResponse::BadRequest()
            .json(internal_err_json("Email/username is beyond 64 length"))
    }

    let query_res = sqlx::query_as!(
        UserLoginEssential,
        "SELECT 
            salt, password, role as \"role:UserRole\" 
        FROM users 
        WHERE email = $1 OR username = $1",
        info.username
    )
    .fetch_one(&app.db)
    .await;


    let queried_cred = match query_res {
        Ok(c) => c,
        Err(e) => {
            
            return HttpResponse::NotFound()
                .json(json!({
                    "status": "error",
                    "message": format!("Failed to query: {}", e)
                }))
        },
    };

    let Ok(hash_pass) = app.hashify_pass(&info.password, &queried_cred.salt) else {
        return HttpResponse::InternalServerError()
            .json(internal_err_json("Failed to hash"));
    };


    if hash_pass != queried_cred.password {
        return HttpResponse::Unauthorized()
            .json(json!({
                "status": "error",
                "message": "Wrong email or password"
            }));
    }

    let Some(claim) = Claim::new(format!("{}:{}", Utc::now(), queried_cred.role)) else {
        return HttpResponse::Unauthorized()
            .json(internal_err_json("Failed to create token"))
    };
    
    let Ok(token) = app.encode_claim(&claim) else {
        return HttpResponse::Unauthorized()
            .json(internal_err_json("Failed to create token"))
    };

    let cookie = CookieBuilder::new("auth_token", token)
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(Duration::days(2))
        .same_site(actix_web::cookie::SameSite::Lax)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({
            "status": "success",
            "message": "successful auth"
        }))
}

#[get("/sign-up")]
async fn sign_up(
    info: web::Json<SignUpUser>,
    app: web::Data<AppState>
) -> impl Responder{



    let user = info.into_inner()
        .build_to_user(app.as_ref());


    let Ok(user) = user else {
        return HttpResponse::InternalServerError()
            .json(internal_err_json("Failed to create hash password"));
    };



    let query_result = sqlx::query!(
        "INSERT INTO users (
            email, password, salt,
            username, first_name, last_name,
            location, phone_no
        )
        VALUES (
            $1, $2, $3,
            $4, $5, $6,
            $7, $8 
        )",
        user.email,
        user.password,
        user.salt,
        user.username,
        user.first_name,
        user.last_name,
        user.location,
        user.phone_no
    )
    .execute(&app.db)
    .await;


    match query_result {
        Ok(_) => {},
        Err(e) => {

            let msg = match e {
                sqlx::Error::Database(e) => {
                    format!(
                        "Database Err - Failed to query: {}", 
                        e.message()
                    )
                },
                e => format!("Failed to query, failed by other factor. \n{e:?}")
            };

            return HttpResponse::InternalServerError()
                .json(internal_err_json(msg));
        },
    }

    HttpResponse::Ok().json(json!({
        "status" : "success",
        "msg" : "successfully added!"
    }))
}


