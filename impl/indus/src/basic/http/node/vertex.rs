use std::net::{ToSocketAddrs};
use warp::{Filter, http::Method, http::HeaderMap};
use warp::filters::path::FullPath;
use url::{Url};

pub async fn run() {
    log::info!("Vertex::run()");
    let indus_top = warp::path::end().map(|| indus_top());

    let webfinger = warp::path!(".well-known" / "webfinger").map(|| webfinger_request());

    let grant_code = warp::path("authorize")
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .and(warp::path::end())
        .and(warp::path::full())
        .and(warp::query::raw())
        .map(|method: Method, headers: HeaderMap, path: warp::path::FullPath, qs: String| authz_code_grant(method,headers, path, qs));
        let routes = warp::any().and(
        warp::get().and(indus_top)
        .or(grant_code)
        .or(warp::get().and(webfinger)));

    let mut adddr_iter = "oauth.indus.in:40401".to_socket_addrs().unwrap();
    warp::serve(routes)
            .run(adddr_iter.next().unwrap())
            .await;
}

fn indus_top() -> String {
    log::info!("indus_top()");
    "Indus OAuth 2.0 and OIDC Identity Provider\n".to_string()
}

fn webfinger_request() -> String {
    log::info!("webfinger()");
    "TODO: webfinger for Indus OpenID Config Service\n".to_string()
}

fn authz_code_grant(mehod: Method, headers: HeaderMap, path: FullPath, qs: String) -> String {
    log::info!("authz_code_grant()");
    if !(mehod == Method::GET || mehod == Method::POST) {
        return "Bad Request Method\n".to_string();
    }
    let mut ps = String::from("https://indus.oauth.in:40401");
    ps.push_str(path.as_str());
    ps.push_str("?");
    ps.push_str(&qs);
    let res = Url::parse(&ps);
    if res.is_ok() {
        res.unwrap().query_pairs().for_each(|(i, x)| log::warn!("{} {}", i, x));
        "TODO: HTTP GET/POST - Authorization Code Grant\n\n".to_string()
    } else {
        "Bad Request\n".to_string()
    }
}
