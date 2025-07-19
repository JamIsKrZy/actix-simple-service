use actix_web::{guard, web, HttpResponse, Responder};
use middleware::auth_check::{AdminValidator, EmployeeValidator, UserValidator};
use serde_json::{Value, json};
use types::{ INTERNAT_ERR_MSG};

use crate::{error::ServiceError, SECRET_KEY};

mod user;
mod auth;
mod bundle;
mod product;

mod util;


type ServiceResult = Result<HttpResponse, ServiceError>;




pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        
        // Temporary routes under /dev
        .service(web::scope("/dev").configure(temporary::scope))


        // User Authenticated routes
        // .service(
        //     web::scope("/user")
        //         .wrap(UserValidator::new(&SECRET_KEY))
        //         .configure(auth::user::scope)
        //         .service(web::scope("/product").configure(product::user::scope)),
        // )




        // Employees Authenticated routes
        .service(
            web::scope("/employee")
                .wrap(EmployeeValidator::new(&SECRET_KEY))
                .configure(product::employee::scope)
        )




        // Admin Authenticated routes
        .service(
            web::scope("/admin")
                .wrap(AdminValidator::new(&SECRET_KEY))
                .configure(product::admin::scope)
                .configure(user::admin::scope)
                .configure(bundle::admin::scope)
        )


        // Public routes
        .service(
            web::scope("")
            .configure(auth::public::scope)
        )

        .default_service(web::to(invalid_url))

    
    ;
        
}


mod temporary{
    use actix_web::{post, web::{self, ServiceConfig}, HttpResponse, Responder};
    use serde_json::json;
    use types::{app_state::AppState, models::init::NewSaltUser as _, payload::admin::NewUser};
    
    use crate::{error::ServiceError, queries::user::insert_one_user};

    pub fn scope(cfg: &mut ServiceConfig){
        cfg
        .service(dev_insert_admin);
    }

    #[post("/{secret_key}")]
    pub async fn dev_insert_admin(
        secret_key: web::Path<String>,
        user_info: web::Json<NewUser>,
        app: web::Data<AppState>
    ) -> Result<impl Responder, ServiceError> {

        if secret_key.as_ref() != "This_is_dev" {
            return Ok(HttpResponse::BadGateway().body("Wrong key"));
        }

        
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






