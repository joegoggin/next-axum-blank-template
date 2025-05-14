use axum::{
    Router,
    http::{HeaderName, Method},
};
use tower_http::cors::{Any, CorsLayer};

use super::message::MessageRouter;

pub struct MainRouter;

impl MainRouter {
    pub fn new() -> Router {
        let cors = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
            .allow_origin(Any)
            .allow_headers([
                HeaderName::from_static("content-type"),
                HeaderName::from_static("authorization"),
            ]);

        Router::new()
            .nest("/message", MessageRouter::new())
            .layer(cors)
    }
}
