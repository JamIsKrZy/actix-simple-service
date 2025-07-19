


mod public {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;
    use types::payload::product::ItemQuery;

    pub fn scope(cfg: &mut web::ServiceConfig) {

    }

    #[get("/discount-list")]
    async fn discount_list() -> impl Responder{
        HttpResponse::NotImplemented()
    }

    #[get("/discount-list")]
    async fn list() -> impl Responder{
        HttpResponse::NotImplemented()
    }

    #[get("/item")]
    async fn item(
        query: web::Query<ItemQuery>
    ) -> impl Responder {
        HttpResponse::NotImplemented()
    }


}


pub mod user {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
        .service(
            web::scope("/product")
            .service(purchased_history)
        )
            
        ;
    }

    #[get("/history")]
    async fn purchased_history() -> impl Responder{
        HttpResponse::NotImplemented()
    }

    #[get("/list")]
    async fn cart_list() -> impl Responder{
        HttpResponse::NotImplemented()
    }

    #[get("/purchased-before")]
    async fn purchased_before_list() -> impl Responder{
        HttpResponse::NotImplemented()
    }


    #[get("/cart/{cart_id}")]
    async fn cart_item() -> impl Responder{
        HttpResponse::NotImplemented()
    }



}


pub mod employee {
    use actix_web::{get, patch, post, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
            .service(set_stock)
            .service(request_unavailable)
        ;
    }


    #[patch("/set-stock/{product_id}")]
    async fn set_stock() -> impl Responder{
        HttpResponse::NotImplemented()
    }

    #[post("/request-unavailable/{product_id}")]
    async fn request_unavailable() -> impl Responder {
        HttpResponse::NotImplemented()
    }


    

}


pub mod admin {
    use actix_web::{get, http::Error, patch, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
    use serde_json::json;
    use types::{app_state::AppState, jwt_ess::Claim, payload::{product::{CreateProduct, EditProduct}, QueryBounds}};

    use crate::{error, queries::{self, product::insert_one_product}, routes::{util, ServiceResult}};

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
        .service(
            web::scope("/product")
            .service(create_product)
            .service(patch_product)
            .service(product_list)
        )
            

        ;
    }


    #[post("/new")]
    async fn create_product(
        req: HttpRequest,
        product: web::Json<CreateProduct>,
        app: web::Data<AppState>
    ) -> Result<HttpResponse, error::ServiceError> {

        let created_by = util::get_claim_id(&req)?;
            
        let final_product = product.into_inner()
            .set_record(&created_by);
        
        insert_one_product(final_product, &app.db).await?;

        Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "product is created!"
        })))
    }

    #[patch("/edit/{product_id}")]
    async fn patch_product(
        id: web::Path<i32>,
        product: web::Json<EditProduct>,
        app: web::Data<AppState>,
        req: HttpRequest
    ) -> Result<HttpResponse, error::ServiceError> {

        let created_by = util::get_claim_id(&req)?;

        let recorded_product = product.into_inner()
            .record_by(created_by);

        queries::product::patch_product(id.into_inner(), recorded_product, &app.db).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success": {
                "message": "successfully edited!"
            }
        })))
        
    }

    #[get("/list")]
    async fn product_list(
        bounds: web::Query<QueryBounds>,
        app: web::Data<AppState>,
    ) -> ServiceResult {

        let bounds = bounds.into_inner().finalize();

        let list = queries::product::product_list(bounds, &app.db).await?;

        Ok(HttpResponse::Ok().json(json!({
            "success" : {
                "message": "",
                "list": list
            }
        })))
    }


}