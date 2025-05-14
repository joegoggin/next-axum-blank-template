use super::server::Server;

pub type AppResult<T> = anyhow::Result<T>;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> AppResult<()> {
        Server::start().await?;

        Ok(())
    }
}
