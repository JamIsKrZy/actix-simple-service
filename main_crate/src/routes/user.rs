



pub mod public {

}

pub mod user {

}


pub mod employee {

}


pub mod admin {
    use actix_web::{get, patch, post, web, HttpResponse, Responder};
    use serde_json::json;
    use types::payload::admin::{EditUser, NewUser};
    use types::{app_state::AppState, payload::QueryBounds};
    use types::models::init::{NewSaltUser, User, UserRole};

    use crate::error::ServiceError;
    use crate::queries::user::{get_list_user, insert_one_user, patch_user};
    
    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
        .service(
            web::scope("/user")
            .service(list_users)
            .service(create_user)
            .service(edit_user)
        )
        ;
    }


    #[post("/new")]
    async fn create_user(
        user_info: web::Json<NewUser>,
        app: web::Data<AppState>
    ) -> Result<impl Responder, ServiceError> {
        
        let user = user_info.into_inner()
            .generate_hash_salt(app.get_ref())
            .map_err(|_| 
                ServiceError::InternalErr("Failed to hash user".to_string()
            ))?;

        insert_one_user(user, &app.db).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success": {
                "message": "Successfully added user!"
            }
        })))
    }

    #[patch("/edit/{user_id}")]
    async fn edit_user(
        user_id: web::Query<String>,
        user_info : web::Json<EditUser>,
        app: web::Data<AppState>
    ) -> Result<impl Responder, ServiceError> {

        patch_user(
            user_id.into_inner(), 
            user_info.into_inner(), 
            &app.db
        ).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success": {
                "message": "Successfully edited user!"
            }
        })))
    }


    #[get("/list")]
    async fn list_users(
        query: web::Query<QueryBounds>, 
        data: web::Data<AppState>
    ) -> Result<impl Responder, ServiceError> {
        let (offset, limit) = query.into_inner().finalize();

        let list = get_list_user(
            (limit, offset), 
            &data.db
        ).await?;

        
        Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "results": list.len(),
            "users": list
        })))
    }

    
}

