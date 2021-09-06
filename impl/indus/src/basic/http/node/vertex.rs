use axum::{
    handler::{get, post},
    http::{Request, StatusCode, header::{HeaderMap, HeaderName, HeaderValue}},
    response::{IntoResponse, Html, Json, Headers},
    Router,
    extract,
    body::{Body, BoxBody},
};
use serde_json::{Value, json};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    log::error!("Vertex CTRL+C signal handler.");
}

pub async fn run() {
    log::info!("Vertex::run()");
    let routes = Router::new()
    .route("/:ignore", get(bad_request))
    .route("/.well-known/webfinger", get(webfinger))
    .route("/:issuer/.well-known/openid-configuration", get(get_issuer_openid_config))
    .route("/.well-known/openid-configuration", get(get_openid_config))
    .route("/", get(bad_request_no_args));
    let sock = axum::Server::bind(&"0.0.0.0:40101".parse().unwrap());
    let server = sock.serve(routes.into_make_service());
    let service = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = service.await {
        log::error!("Vertex Runtime Error: {}", e);
    }
}

async fn webfinger() -> String {
    log::warn!("webfinger()");
    "TODO: webfinger for Indus OpenID Config Service".to_string()
}

async fn get_openid_config() -> (StatusCode, HeaderMap, Json<Value>) {
    log::info!("get_openid_config()");
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
    headers.insert(
        axum::http::header::PRAGMA,
        HeaderValue::from_static("no-cache"),
    );
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    (StatusCode::NOT_IMPLEMENTED, headers, Json(json!({ "iss": "OpenID Configuration".to_string() })))
}

async fn get_issuer_openid_config(extract::Path(issuer): extract::Path<String>) -> String {
    log::warn!("get_issuer_openid_config");
    "TODO: issuer specific OpenID configuration discovery for '".to_string() + &issuer + &"'"
}

async fn bad_request(extract::Path(badurl): extract::Path<String>) -> String {
    log::error!("bad_request: {}", &badurl);
    "Bad Request".to_string()
}
async fn bad_request_no_args() -> String {
    log::error!("bad_request");
    "Bad Request".to_string()
}
