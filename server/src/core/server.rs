use anyhow::Error;
use axum::serve;
use tokio::net::TcpListener;

use crate::routes::main::MainRouter;

use super::app::AppResult;

pub struct Server;

impl Server {
    pub async fn start() -> AppResult<()> {
        let listener = TcpListener::bind("0.0.0.0:8000").await?;
        let rotuer = MainRouter::new();
        let server = serve(listener, rotuer).await;

        match server {
            Ok(_) => println!("Server listening on port 8000"),
            Err(_) => return Err(Error::msg("Failed to start server")),
        }

        Ok(())
    }
}
