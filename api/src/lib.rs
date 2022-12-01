#![allow(dead_code)] // todo remove this at some point

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

use actix_cors::Cors;

pub mod prelude {
    pub use anyhow::{anyhow, Result};
}

use async_graphql::Schema;
use model::{room::Room, user::User};
use store::DataStore;
use stream::StreamController;

pub async fn start_api_server<Fut>(callback: Option<impl FnOnce() -> Fut>) -> std::io::Result<()>
where
    Fut: Future<Output = ()>,
{
    let user_store = DataStore::<User>::new();
    let room_store = DataStore::<Room>::new();
    let stream_ctl = StreamController::new();
    let orbt_data =
        server::OrbtData::new(user_store.clone(), room_store.clone(), stream_ctl.clone());
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
            std::fs::write("./orbt-version.txt", version)?;
            return Ok(());
        }
    }

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .app_data(Data::new(orbt_data.clone()))
            .app_data(Data::new(user_store.clone()))
            .app_data(Data::new(room_store.clone()))
            .app_data(Data::new(stream_ctl.clone()))
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
