use axum::serve;
use tokio::net::TcpListener;

use crate::routes::main::MainRouter;

use super::app::AppResult;

pub struct Server;

impl Server {
    pub async fn start() -> AppResult<()> {
        println!("Server running on port 8000");

        let listener = TcpListener::bind("0.0.0.0:8000").await?;
        let rotuer = MainRouter::new();

        serve(listener, rotuer).await?;

        Ok(())
    }
}
