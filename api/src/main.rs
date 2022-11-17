mod server;
mod model;
mod schema;
mod store;
mod types;
mod auth;

pub mod prelude {
    pub use anyhow::{Result, anyhow};
}

use actix_web::{HttpServer, App, web::{Data, self}, guard};
use dotenv::dotenv;
use store::DataStore;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let data_store = DataStore::new();
    let orbt_data = server::OrbtData::new(data_store.clone());
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(orbt_data.clone()))
            .app_data(Data::new(data_store.clone()))
            .service(web::resource("/").guard(guard::Post()).to(server::graphql_root))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(server::graphql_ws)
            )
            .service(web::resource("/").guard(guard::Get()).to(server::graphql_playground))
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}