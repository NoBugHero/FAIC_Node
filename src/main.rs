use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod api;
mod blockchain;
mod network;
mod wallet;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting FAIC Node...");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            // 钱包相关路由
            .service(
                web::scope("/api")
                    .route("/wallet/create", web::post().to(api::create_wallet))
                    .route("/wallet/import", web::post().to(api::import_wallet))
                    .route("/wallet/{address}/balance", web::get().to(api::get_balance))
                    .route("/wallet/{address}/transactions", web::get().to(api::get_transaction_history))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}