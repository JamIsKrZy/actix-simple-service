


mod public {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;
    use types::schemas::product::ItemQuery;

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
            .service(purchased_history)

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
    use actix_web::{get, http::Error, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
    use serde_json::json;
    use types::{jwt_ess::Claim, schemas::product::CreateProduct};

    use crate::routes::util;

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
            .service(create_product)
            .service(edit_product)

        ;
    }


    use util::Error as UtilErr;

    #[post("/new")]
    async fn create_product(
        req: HttpRequest,
        product: web::Json<CreateProduct>
    ) -> Result<HttpResponse, UtilErr> {

        let created_by = util::claim_ref(&req)?;
            
        



        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "product is created!"
        }))
    }

    #[post("/edit/{product_id}")]
    async fn edit_product(
        id: web::Path<i32>
    ) -> impl Responder {
        HttpResponse::NotImplemented()
    }

    



}