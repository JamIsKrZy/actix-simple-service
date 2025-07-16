


mod public {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {

    }
}


mod user {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {

    }
}


mod employee {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
            .service(
                web::scope("/employe")
            )
        ;
    }



}


mod admin {
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde_json::json;

    pub fn scope(cfg: &mut web::ServiceConfig) {
        cfg
            .service(
                web::scope("/admin")
                    .service(create_product)
                    .service(edit_product)
            )

        ;
    }

    #[post("/new-product")]
    async fn create_product() -> impl Responder {
        HttpResponse::NotImplemented()
    }

    #[post("/edit-product/{product_id}")]
    async fn edit_product() -> impl Responder {
        HttpResponse::NotImplemented()
    }

    



}