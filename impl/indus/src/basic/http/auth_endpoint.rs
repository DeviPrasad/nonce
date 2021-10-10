use std::net::{ToSocketAddrs};
use warp::{Filter, http::Method, http::HeaderMap};
use warp::filters::path::FullPath;
use url::{Url};
use std::net::SocketAddr;

#[derive(std::fmt::Debug)]
pub enum VertexError {
    None,
    BadSocket,
    BadAddrString,
}

#[derive(std::fmt::Debug)]
pub struct AuthEndpoint {
    running: bool,
    err: VertexError,
    host_port: String,
    sock: Option<SocketAddr>,
}

impl AuthEndpoint {
    pub fn new(running: bool, err: VertexError, host_port: String, sock: Option<SocketAddr>) -> AuthEndpoint {
        AuthEndpoint {running, err, host_port, sock}
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

    fn init(host_addr: &str) -> AuthEndpoint {
        let sock_addresses = host_addr.to_socket_addrs();
        let mut err = VertexError::BadAddrString;
        let mut sock = None;
        if sock_addresses.is_ok() {
            let mut adddr_iter = sock_addresses.unwrap();
            err = VertexError::BadSocket;
            sock = adddr_iter.next();
            if sock.is_some() {
                err = VertexError::None;
            }
        }
        AuthEndpoint::new(false, err, String::from(host_addr), sock)
    }

    pub async fn run(&self) {
        let indus_top = warp::path::end().map(|| AuthEndpoint::indus_top());
        let webfinger = warp::path!(".well-known" / "webfinger").map(|| AuthEndpoint::webfinger_request());
        let grant_code = warp::path("authorize")
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(warp::path::end())
            .and(warp::path::full())
            .and(warp::query::raw())
            .map(|method: Method, headers: HeaderMap, path: warp::path::FullPath, qs: String| AuthEndpoint::authz_code_grant(method,headers, path, qs));
        let routes = warp::any().and(
            warp::get().and(indus_top)
            .or(grant_code)
            .or(warp::get().and(webfinger)));
        warp::serve(routes).run(self.sock.unwrap()).await;
    }

    pub async fn start() {
        log::info!("Vertex::start()");
        let auth = AuthEndpoint::init("oauth.indus.in:40401");
        match auth.err {
            VertexError::None => {
                log::info!("Assigned authorization endpoint <{}>", auth.sock.unwrap());
                auth.run().await;
                log::info!("Stopped authorization endpoint <{}>", auth.sock.unwrap())
            }
            _  => {
                log::error!("Start authorization endpoint failed <{}>:<{:#?}>", auth.host_port, auth.err);
            }
        }
        log::error!("Quitting...");
    }
}
