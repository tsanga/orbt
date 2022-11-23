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

use async_graphql::Schema;
use store::DataStore;

pub async fn start_api_server<Fut>(callback: Option<impl FnOnce() -> Fut>) -> std::io::Result<()>
where
    Fut: Future<Output = ()>,
{
    let data_store = DataStore::new();
    let orbt_data = server::OrbtData::new(data_store.clone());
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1].to_lowercase() == "--export-sdl" {
            println!("Exporting GraphQL SDL.");
            let schema = Schema::build(
                schema::Query::default(),
                schema::Mutation::default(),
                schema::Subscription::default(),
            )
            .finish();
            std::fs::write("./orbt.graphql", schema.sdl())?;
            return Ok(());
        } else if args[1].to_lowercase() == "--export-version" {
            let version = env!("CARGO_PKG_VERSION").to_string();
            println!("{}", &version);
            std::fs::write("./flux-version.txt", version)?;
            return Ok(());
        }
    }

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