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
    scheme: String,
}
impl AuthEndpoint {
    pub fn new(running: bool, err: VertexError, host_port: String, sock: Option<SocketAddr>, scheme: &str) -> AuthEndpoint {
        AuthEndpoint {running, err, host_port, sock, scheme: String::from(scheme)}
    }
    fn pleb_consent() -> warp::reply::Response {
        let uri = Uri::from_static("basic.html");
        log::info!("consent - sending redirect {:#?}", uri);
        warp::redirect::temporary(uri).into_response()
    }

    fn access_volatile(res: String, id: String) -> warp::reply::Response {
        log::info!("access_volatile - resource {} and id {} {}", res, id, "http://".to_string() + &res);
        let reply = warp::reply::json(&serde_json::json!({}));
        let reply = warp::reply::with_status(reply, StatusCode::from_u16(200).unwrap());
        let reply = warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*");
        let reply = warp::reply::with_header(reply, "Vary", "Origin");
        let reply = warp::reply::with_header(reply, "Access-Control-Allow-Methods", "GET, POST");
        let reply = warp::reply::with_header(reply, "x-indus-volatile-nonce", "CADFEED");
        let reply = warp::reply::with_header(reply, "Location", "http://".to_string() + &res);
        let loc = String::from("https://") + &res;
        let reply = warp::redirect::temporary(Uri::from_static("http://peer.authn.com:60000/authorize"));
        warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
        .into_response()
        //log::info!("access_volatile - response {:#?}", reply.into_response());
        //reply.into_response()
    }

    fn rs_custom_consent() -> warp::reply::Response {
        let uri = Uri::from_static("action/");
        log::info!("rs_custom_consent - sending redirect {:#?}", uri);
        warp::redirect::temporary(uri).into_response()
    }

    fn authz_code_grant(_mehod: Method, headers: HeaderMap, path: FullPath, qs: String) -> warp::reply::Response {
        log::info!("authz_code_grant()");
        for (key, value) in headers.iter() {
            println!("hk: {:?}: val: {:?}", key, value);
        }
        // example "http://loiter.xyz.in:45001"
        let host_header: &HeaderValue = headers.get("Host").unwrap();
        let host = host_header.to_str().unwrap();
        let mut ps = "http://".to_string() + host + "/";
        ps.push_str(path.as_str());
        ps.push_str("?");
        ps.push_str(&qs);
        let res = Url::parse(&ps);
        if res.is_ok() {
            let cluri = res.unwrap();
            let authority = host;
            println!("{} -- {} -- {:?}", cluri, cluri.scheme(), authority);
            //res.unwrap().query_pairs().for_each(|(i, x)| log::warn!("{} {}", i, x));
            let _uri = Uri::builder().scheme(cluri.scheme())
            .authority(authority) //"loiter.xyz.in:45001")
            .path_and_query(String::from("/pkce/code/redirect?") + &qs + "&code=eyTxz987NqwpfgjSvn")
            .build();
            log::info!("authz_code_grant() - sent redirect {:#?}", _uri);
            //warp::redirect::temporary(_uri.unwrap()).into_response()
            //let uri = Uri::from_static("consent");
            let uri = Uri::from_static("http://peer.authn.com:60000/authorize");
            //let uri = Uri::from_static("/virt/peer.authn.com:60000");
            warp::redirect::see_other(uri).into_response()
        } else {
            warp::reply::with_status(warp::reply::json(&""),
                StatusCode::from_u16(400).unwrap()).into_response()
        }
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

    pub async fn run(self: &AuthEndpoint) {
        let scheme = self.scheme.clone();
        let cors = warp::cors()
            //.allow_origin("http://loiter.xyz.in:45001")
            .allow_any_origin()
            .allow_headers(vec!["content-type", "User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"]);
        //let webf = warp::path!(".well-known" / "webfinger").map(|| AuthEndpoint::webfinger_request());
        let consent = warp::get()
            .and(warp::path("consent"))
            .and(warp::path::end())
            .map(|| log::info!("CONSENT_ASK")).untuple_one()
            .map(|| AuthEndpoint::pleb_consent())
            .with(cors.clone());
        let code_grant = warp::get()
            .and(warp::path("authorize"))
            .and(warp::path::end())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(warp::path::full())
            .and(warp::query::raw())
            .map(|method: Method, headers: HeaderMap, path: warp::path::FullPath, qs: String| { AuthEndpoint::authz_code_grant(method, headers, path, qs) } )
            .with(cors.clone());
        let volatile = warp::get()
            .and(warp::path!("virt" / String / String))
            .map(|res: String, id: String| AuthEndpoint::access_volatile(res, id))
            .with(cors.clone());
        let fs = warp::any()
            //.and(warp::path::end())
            //.and(warp::path::full())
            //.map(|p: _| log::info!("static asseet {:? }", p)).untuple_one()
            .and(warp::fs::dir("static"))
            //.and(warp::filters::
            .map(|f: _| {
                let r = warp::reply::with_header(f, "Location", "http://oauth.indus.in:40401/basic.html");
                warp::reply::with_status(r, StatusCode::from_u16(200).unwrap())
                //warp::reply::with_status(r, StatusCode::from_u16(303).unwrap())
            })
            .with(cors.clone());

        //let routes = warp::any().and(grant_code)
        let routes = code_grant
                .or(consent)
                .or(volatile)
                .or(fs);
        warp::serve(routes).run(self.sock.unwrap()).await;
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
