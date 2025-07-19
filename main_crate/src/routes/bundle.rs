

pub mod public {

}

pub mod user {
    
}

pub mod employee {
    
}

pub mod admin {
    use actix_web::{cookie::time::util, dev::ServiceResponse, get, post, web::{self, ServiceConfig}, HttpRequest, HttpResponse};
    use serde_json::json;
    use types::{app_state::AppState, payload::{bundle::CreateBundle, QueryBounds}};

    use crate::{queries::{self, bundle::insert_one_bundle}, routes::{util::get_claim_id, ServiceResult}};

    

    pub fn scope(cfg: &mut ServiceConfig){
        cfg
        .service(
            web::scope("/bundle")
            .service(create_bundle)
            .service(edit_bundle)
            .service(bundle_list)
        )
        ;
    }

    #[post("/new")]
    async fn create_bundle(
        bundle: web::Json<CreateBundle>,
        app: web::Data<AppState>,
        req: HttpRequest
    ) -> ServiceResult {
        
        let created_by = get_claim_id(&req)?;

        let recorded_bundle = bundle.into_inner().record(created_by);

        insert_one_bundle(recorded_bundle, &app.db).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success": {
                "message": "bundle succesfully added!"
            }
        })))
    }

    #[post("/edit/{bundle_id}")]
    async fn edit_bundle(
        id: web::Path<i64>
    ) -> ServiceResult {
        Ok(HttpResponse::NotImplemented().body(""))
    }

    #[get("/list")]
    async fn bundle_list(
        bounds: web::Query<QueryBounds>,
        app: web::Data<AppState>
    ) -> ServiceResult {

        let bundle_list = queries::bundle::bundle_list(
            bounds.into_inner(), 
            &app.db
        ).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success": {
                "bundles": bundle_list,
                "message": ""
            }
        })))
    }

}