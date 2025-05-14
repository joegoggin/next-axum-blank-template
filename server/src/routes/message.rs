use axum::{Router, routing::get};

use crate::controllers::message::MessageController;

pub struct MessageRouter;

impl MessageRouter {
    pub fn new() -> Router {
        Router::new().route("/", get(MessageController::get()))
    }
}
