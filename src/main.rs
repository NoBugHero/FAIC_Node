use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use env_logger::{Builder, Target};
use log::info;

mod api;
mod blockchain;
mod network;
mod wallet;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置日志
    Builder::new()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Debug)
        .init();
        
    // 在关键位置添加日志
    info!("FAIC Node starting...");

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
                    .route("/transfer", web::post().to(api::transfer))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}