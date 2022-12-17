#![allow(dead_code)] // todo remove this at some point

pub mod prelude {
    pub use anyhow::{anyhow, Result};
}

use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    api::start_api_server(Some(|| async {
        let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
        let port = std::env::var("PORT").unwrap_or("8080".to_string());
        println!("[orbt:api] Started. Listening on {}:{}", address, port);
    }))
    .await
}
