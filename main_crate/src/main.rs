use std::{ sync::LazyLock};

use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use types::app_state::AppState;

mod routes;
mod queries;
mod error;
mod tests;


static SECRET_KEY: LazyLock<String> = LazyLock::new(|| {
    dotenvy::var("SECRET_KEY").expect("Secret Key is undefined in environment")
});




fn init_ssl() -> SslAcceptorBuilder {
    let mut ssl_builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file("keys/key.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder
        .set_certificate_chain_file("keys/cert.pem")
        .unwrap();

    ssl_builder
}

fn init_env() {
    dotenvy::dotenv().expect("Failed to set dot_envy");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();

    let appstate = web::Data::new(
        AppState::init(&SECRET_KEY).await
        .expect("Failed to set up appstate")
    );
    
    let ssl_builder = init_ssl();


    HttpServer::new(move || {
        App::new()
            .app_data(appstate.clone())
            .wrap(Logger::default())
            .configure(routes::routes)
    })
    .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .workers(2)
    .run()
    .await
}
