use actix_web::{web};




pub fn scope(cfg: &mut web::ServiceConfig){
    use guest_services as gs;

    cfg
        .service(gs::available_service_list)
        .service(gs::highlight_services)
        .service(gs::discount_services)

    ;
}

mod guest_services{
    use actix_web::{get, HttpResponse, Responder};
    use serde_json::json;
    
    #[get("/service-list")]
    async fn available_service_list() -> impl Responder {

        HttpResponse::Accepted().json(json!({
            "available-list": [
                "Message", 
                "milktea"
            ]
        }))
    }

    #[get("/highlight-list")]
    async fn highlight_services() -> impl Responder {
        HttpResponse::NotImplemented()
    }

    #[get("/discount-list")]
    async fn discount_services() -> impl Responder {
        HttpResponse::NotImplemented()
    }
    
}

mod user_service {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/for-you-list")]
    async fn for_you_service() -> impl Responder {
        HttpResponse::NotImplemented()
    }

    #[get("/scheduled-list")]
    async fn scheduled_service() -> impl Responder {
        HttpResponse::NotImplemented()
    }


    async fn order_service() -> impl Responder {
        HttpResponse::NotImplemented()
    }

}

