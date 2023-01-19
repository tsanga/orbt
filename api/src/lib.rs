#![allow(dead_code)] // todo remove this at some point

mod auth;
mod model;
mod schema;
mod server;
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
use server::OrbtData;
use stream::StreamControl;

pub type Database = musty::Musty<mongodb::Database>;

pub async fn start_api_server<Fut>(callback: Option<impl FnOnce() -> Fut>) -> std::io::Result<()>
where
    Fut: Future<Output = ()>,
{
    let stream_user_room_ctl = StreamControl::<User, Room>::new();
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

    let db = musty::prelude::Musty::new(mongodb::Client::with_uri_str(
        &std::env::var("MONGODB_URI").unwrap_or_else(|f| "mongodb://localhost:27017".to_string()),
    )
    .await
    .unwrap()
    .database("orbt")); 

    let app_data = OrbtData::new(db.clone(), stream_user_room_ctl.clone());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .app_data(Data::new(app_data.clone()))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(stream_user_room_ctl.clone()))
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
