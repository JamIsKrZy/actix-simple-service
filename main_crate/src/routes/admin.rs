


use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use types::schemas::query::{ListAmount};
use types::{AppState, INTERNAT_ERR_MSG};
use types::models::user::{User, UserRole};

pub fn scope(cfg: &mut web::ServiceConfig){
    cfg.service(
            web::scope("/admin")
                .service(list_users)
    );
}

#[get("/user-list")]
async fn list_users(
    query: web::Query<ListAmount>,
    data: web::Data<AppState>
) -> impl Responder{
    let limit = query.max.unwrap_or(25);
    let offset = query.offset.unwrap_or(0).saturating_sub(1);

    let query = sqlx::query_as!(
        User,
        "SELECT 
            id,email,password, 
            salt, username, first_name, 
            last_name, location, phone_no,
            role as \"role:UserRole\" 
        FROM users 
        ORDER BY username 
        LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if let Ok(list) = query{

        HttpResponse::Ok().json(json!({
            "status": "success",
            "results": list.len(),
            "users": list
        }))
    } else {
        HttpResponse::InternalServerError().json(json!({
            "status": "err",
            "message": INTERNAT_ERR_MSG.to_owned() + "failed to fetch user list"
        }))
    }
    
}


async fn create_admin() -> impl Responder{
    HttpResponse::NotImplemented()
}

async fn create_staff() -> impl Responder{
    HttpResponse::NotImplemented()
}

async fn create_service() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn edit_service() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn delete_service() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn set_highlight_services() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn set_unhighlight_service() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn set_discount_service() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn remove_discount_service() -> impl Responder {
    HttpResponse::NotImplemented()
}
