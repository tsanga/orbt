pub mod prelude {
    pub use anyhow::{anyhow, Result};
}

use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    api::start_api_server(Some(|| async {
        println!("Server started!");
    }))
    .await
}
