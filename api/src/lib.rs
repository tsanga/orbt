#![feature(let_else)]

mod auth;
mod model;
mod schema;
mod server;
mod store;
mod stream;
mod types;

use std::future::Future;

use actix_web::{
    guard,
    web::{self, Data},
    App, HttpServer,
};

pub mod prelude {
    pub use anyhow::{anyhow, Result};
}

use store::DataStore;

pub async fn start_api_server<Fut>(callback: Option<impl FnOnce() -> Fut>) -> std::io::Result<()>
where
    Fut: Future<Output = ()>,
{
    let data_store = DataStore::new();
    let orbt_data = server::OrbtData::new(data_store.clone());
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(orbt_data.clone()))
            .app_data(Data::new(data_store.clone()))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(server::graphql_root),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(server::graphql_ws),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(server::graphql_playground),
            )
    })
    .bind(format!("{}:{}", address, port))?
    .run();

    if let Some(callback) = callback {
        futures::future::join(server, callback()).await.0
    } else {
        server.await
    }
}
