use std::convert::Infallible;
use std::net::{ToSocketAddrs};
use url::{Url};
use std::net::SocketAddr;
mod pleb;
use pleb::logger;
//use bytes::Bytes;
use std::collections::HashMap;
use url::form_urlencoded::parse;

use hyper::{
    Uri,
//    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, Method,
    HeaderMap,
};
use hyper::server::conn::AddrStream;
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
    scheme: String,
}

impl AuthEndpoint {
    pub fn new(running: bool, err: VertexError, host_port: String, sock: Option<SocketAddr>, scheme: &str) -> AuthEndpoint {
        AuthEndpoint {running, err, host_port, sock, scheme: String::from(scheme)}
    }
    fn init(scheme: &str, host_addr: &str) -> AuthEndpoint {
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
            AuthEndpoint::new(false, err, String::from(host_addr), sock, scheme)
    }
    fn fnonce() -> HashMap<String, String> {
        println!("fnonce");
        HashMap::new()
    }
    fn query_params(req: &Request<Body>) {
        let params: HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    println!("v = {}", v);
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(AuthEndpoint::fnonce);
                //.unwrap_or_else(HashMap::new);

        for (_key, _value) in params {
            println!("query_params: {:?}: val: {:?}", _key, _value);
        }
    }
    pub async fn run(self: &AuthEndpoint) {
        //let scheme = self.scheme.clone();
        async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
            let headers: &HeaderMap = req.headers();
            for (_key, _value) in headers.iter() {
                //println!("hk: {:?}: val: {:?}", key, value);
            }
            AuthEndpoint::query_params(&req);
            let path: &str = req.uri().path();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/authorize") => {
                    let host_name_port: &str = headers.get("Host").unwrap().to_str().unwrap();
                    let url_str = "http://".to_string() + host_name_port + &req.uri().to_string();
                    let req_url = Url::parse(&url_str).unwrap();
                    let params = req_url.query_pairs();
                    if let Some(qs) = req.uri().query() {
                        log::info!("request path and query: {} {}", path, qs);
                        params.for_each(|(i, x)| log::info!("{} {}", i, x));
                    }
                    Ok(Response::new(Body::from("<html><body><h1>Hello</h1></body></html>")))
                },
                (_, _) => {
                    Ok(Response::new(Body::from("<html><body><h1>Bad Request</h1></body></html>")))
                }
            }
        }
        let new_service = make_service_fn(|_conn: &AddrStream| async {
            Ok::<_, Infallible>(service_fn(handle))
        });
        let server = Server::bind(&self.sock.unwrap()).serve(new_service);
        println!("Listening on http://{}", self.sock.unwrap());
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }

    pub async fn start() {
        log::info!("Vertex::start()");
        let auth: AuthEndpoint = AuthEndpoint::init("http", "oauth.indus.in:40401");
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

#[tokio::main]
async fn main() {
    if !logger::init_logging() {
        println!("Error - Logger init failed. Quitting. ");
        std::process::exit(1);
    }
    log::info!("Starting Barebone System");
    AuthEndpoint::start().await;
}
