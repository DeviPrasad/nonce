use std::net::{ToSocketAddrs};
use warp::{http::StatusCode,  http::Uri, Filter, http::Method, http::HeaderMap, http::HeaderValue};
use warp::filters::path::FullPath;
use url::{Url};
use std::net::SocketAddr;
use warp::Reply;

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
    fn authz_code_grant(mehod: Method, headers: HeaderMap, path: FullPath, qs: String) -> warp::reply::Response {
        log::info!("authz_code_grant()");
        for (key, value) in headers.iter() {
            println!("hk: {:?}: val: {:?}", key, value);
        }
        // example "http://loiter.xyz.in:45001"
        let origin_header: &HeaderValue = headers.get("Origin").unwrap();
        let origin = origin_header.to_str().unwrap();
        let mut ps = String::from(origin);
        ps.push_str(path.as_str());
        ps.push_str("?");
        ps.push_str(&qs);
        let res = Url::parse(&ps);
        if res.is_ok() {
            let cluri = res.unwrap();
            let authority = origin.strip_prefix(&(String::from(cluri.scheme()) + "://"));
            println!("{} -- {} -- {:?}", cluri, cluri.scheme(), authority);
            //res.unwrap().query_pairs().for_each(|(i, x)| log::warn!("{} {}", i, x));
            let uri = Uri::builder().scheme(cluri.scheme())
                .authority(authority.unwrap()) //"loiter.xyz.in:45001")
                .path_and_query(String::from("/pkce/code/redirect?") + &qs + "&code=eyTxz987NqwpfgjSvn")
                .build();
            log::info!("authz_code_grant() - sent redirect {:#?}", uri);
            warp::redirect::temporary(uri.unwrap()).into_response()
        } else {
            warp::reply::with_status(warp::reply::json(&""),
                StatusCode::from_u16(400).unwrap()).into_response()
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
        let cors = warp::cors()
            //.allow_origin("http://loiter.xyz.in:45001")
            .allow_any_origin()
            .allow_headers(vec!["content-type", "User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"]);
        //let webfinger = warp::path!(".well-known" / "webfinger").map(|| AuthEndpoint::webfinger_request());
        let grant_code = warp::path("authorize")
            .and(warp::method())
            .and(warp::header::headers_cloned())
            //.and(warp::filters::header::value("origin"))
            .and(warp::path::end())
            .and(warp::path::full())
            .and(warp::query::raw())
            .map(|method: Method, headers: HeaderMap, path: warp::path::FullPath, qs: String| AuthEndpoint::authz_code_grant(method, headers, path, qs))
            .with(cors).with(warp::log("grant_code - cors request"));
        let routes = warp::any().and(grant_code);
                //.and(warp::get().and(indus_top)
                //.or(grant_code)
                //.or(warp::get().and(webfinger)));
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
