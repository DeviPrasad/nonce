use bytes::Bytes;
use axum::{
    handler::{get, post},
    http::Request,
    http::StatusCode,
    Router,
    extract,
    body::{Body, BoxBody},
};
use serde::Deserialize;
use std::{io};

pub async fn run() {
    log::info!("Vertex::run()");
    let app = Router::new().route("/", get(handler));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> String {
    log::warn!("handler()");
    "Hello, World!".to_string()
}
