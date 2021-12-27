#![crate_name = "barebone"]
#[doc(inline)]
use std::convert::Infallible;
use std::net::{ToSocketAddrs};
use std::net::SocketAddr;
use std::fmt;
use std::str;
mod pleb;
use pleb::logger;
use http::StatusCode;
use http::header;
use bytes::Bytes;
use http_body::{Body as HttpBody};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
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
    err: VertexError,
    host_port: String,
    sock: Option<SocketAddr>,
}

#[derive(std::fmt::Debug)]
pub struct QueryParams {
    pub(self) params: Vec<(String, String)>,
}

impl QueryParams {
    pub fn new(params: Vec<(String, String)>) -> QueryParams {
        QueryParams { params }
    }

    pub fn from_uri(uri: &http::Uri) -> QueryParams {
        let mut params: Vec<(String, String)> = uri.query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes()).into_owned().collect()
            })
            .unwrap_or_else(Vec::new);
        for (k,v) in params.iter_mut() {
            k.make_ascii_lowercase();
        }
        params.sort_by(|(x, _), (a, _)| x.cmp(a));
        QueryParams::new(params)
    }
    pub fn from_str(qs: &str) -> QueryParams {
        let mut params: Vec<(String, String)> =
                url::form_urlencoded::parse(qs.as_bytes()).into_owned().collect();
        for (k,v) in params.iter_mut() {
            k.make_ascii_lowercase();
        }
        params.sort_by(|(x, _), (a, _)| x.cmp(a));
        QueryParams::new(params)
    }
    pub fn find(&self, name: &String) -> Option<&String> {
        if let Some((_, v)) = self.params.iter().find(|(kn, _)| name == kn) {
            Some(v)
        } else {
            None
        }
    }
    pub fn state(&self) -> Option<&String> {
        self.find(&"state".to_string())
    }
    pub fn nonce(&self) -> Option<&String> {
        self.find(&"nonce".to_string())
    }
    pub fn redirect_url(&self) -> Option<&String> {
        self.find(&"redirect_url".to_string())
    }
    pub fn client_id(&self) -> Option<&String> {
        self.find(&"client_id".to_string())
    }
    pub fn scopes(&self) -> Vec<&str> {
        self.find(&"scope".to_string()).map(|s| s.split_whitespace().collect()).unwrap_or_else(Vec::new)
    }
    pub fn resource(&self) -> Vec<&str> {
        let resources = self.params.iter().filter(|(k, _)| k.eq("resource"))
            .map(|(_, v)| v.as_str()).collect();
        println!("resources: {:?}", resources);
        resources
    }
}

#[derive(std::fmt::Debug)]
pub struct Headers {
    pub(self) headers: HeaderMap,
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, val) in &self.headers {
            if  !val.is_empty() {
                if let Ok(s) = str::from_utf8(val.as_bytes()) {
                    write!(f, "({} : {})", name.as_str(), s).unwrap();
                }
            }
        }
        Ok(())
    }
}

impl Headers {
    pub fn from(hm: HeaderMap) -> Headers {
        Headers { headers: hm }
    }
    pub fn probe(&self, name: &str) -> Option<&str> {
        for (key, val) in &self.headers {
            if name == key { return str::from_utf8(val.as_bytes()).ok() }
        }
        None
    }
    pub fn authz_basic(&self) -> Result<&str, bool> {
        if self.headers.contains_key(header::AUTHORIZATION) {
            let val = &self.headers[header::AUTHORIZATION];
            if let Ok(hval) = str::from_utf8(val.as_bytes()) {
                let mut authz = hval.split_ascii_whitespace();
                if let Some(authz_type) = authz.next() {
                    if authz_type.to_ascii_lowercase() != "basic" { return Err(false); }
                    let cred = authz.next();
                    return cred.ok_or(false)
                }
            }
        }
        Err(false)
    }
}

#[derive(std::fmt::Debug)]
pub struct RequestBody {
    pub(self) payload: String,
    pub(self) json: serde_json::Value,
}

impl RequestBody {
    const PAYLOAD_SIZE_MAX: u64 = (32 * 1024);
    pub fn from_bytes(body: Bytes) -> RequestBody {
        if !body.is_empty() {
            if let Ok(form_body) = String::from_utf8(body.to_vec()) {
                return  RequestBody {
                    payload: form_body.to_owned(),
                    json: serde_json::from_str(&form_body).unwrap()
                }
            }
        }
        RequestBody {payload : String::from(""), json: serde_json::Value::from("") }
    }
}

#[derive(std::fmt::Debug)]
pub struct ErrorResponse {
    body: ResponseBody,
    code: String,
    desc: String,
    uri: String,
    state: Vec<u8>,
}


/// OAuth 2.0 and OIDC error codes per respective specs.
/// RFC 6749, section 4.1.2.1
impl ErrorResponse {
    const ERR_CODE_EMPTY: &'static str = "";
    const ERR_INVALID_REQUEST: &'static str = "invalid_request";
    const ERR_UNAUTHZ_CLIENT: &'static str = "unauthorized_client";
    const ERR_INVALID_CLIENT: &'static str = "invalid_client";
    const ERR_ACCESS_DENIED: &'static str = "access_denied";
    const ERR_UNSUPPORTED_RESPONSE_TYPE: &'static str = "unsupported_response_type";
    const ERR_INVALID_SCOPE: &'static str = "invalid_scope";
    const ERR_SERVER_ERROR: &'static str = "server_error";
    const ERR_TEMP_UNAVAILABLE: &'static str = "temporarily_unavailable";
    const ERR_INVALID_GRANT: &'static str = "invalid_grant";
    const ERR_UNSUPPORTED_GRANT_TYPE: &'static str = "unsupported_grant_type";

    fn new() -> ErrorResponse {
        ErrorResponse {
            body : ResponseBody::new(),
            code: ErrorResponse::ERR_CODE_EMPTY.to_string(),
            desc: ErrorResponse::ERR_CODE_EMPTY.to_string(),
            uri: ErrorResponse::ERR_CODE_EMPTY.to_string(),
            state: Vec::new(),
        }
    }
    fn safe_uri(uri: &str) -> bool {
        if uri.is_ascii() {
            for ch in uri.chars() {
                let r = match ch as u8 {
                    0x21 => true,
                    0x23 ..= 0x5B => true,
                    0x5D ..= 0x7E => true,
                    _ => false
                };
                if !r { return false; }
            }
        }
        true
    }
    fn safe_str(uri: &str) -> bool {
        if uri.is_ascii() {
            for ch in uri.chars() {
                let r = match ch as u8 {
                    0x21 => true,
                    0x23 ..= 0x5B => true,
                    0x5D ..= 0x7E => true,
                    _ => false
                };
                if !r { return false; }
            }
        }
        true
    }
    /// RFC 6749 states that values for the "error" parameter MUST NOT include
    /// these three ASCII chars: ", \, and DEL.
    ///
    /// updates the response object only if `code` connforms.
    fn set_error_code(&mut self, code: &str) -> &mut ErrorResponse {
        if code.is_ascii() && ErrorResponse::safe_str(code) {
            self.code = code.to_owned();
        }
        self
    }
    fn invalid_request(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_INVALID_REQUEST)
    }
    fn invalid_client(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_INVALID_CLIENT)
    }
    fn unauthorized_client(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_UNAUTHZ_CLIENT)
    }
    fn invalid_grant(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_INVALID_GRANT)
    }
    fn access_denied(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_ACCESS_DENIED)
    }
    fn unsupported_response_type(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_UNSUPPORTED_RESPONSE_TYPE)
    }
    fn invalid_scope(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_INVALID_SCOPE)
    }
    fn server_error(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_SERVER_ERROR)
    }
    fn temporarily_unavailable(&mut self) -> &mut ErrorResponse {
        self.set_error_code(ErrorResponse::ERR_TEMP_UNAVAILABLE)
    }
    /// updates the response object only if `desc` connforms.
    fn set_description(&mut self, desc: &str) -> &mut ErrorResponse {
        if desc.is_ascii() && ErrorResponse::safe_str(desc) {
            self.desc = desc.to_owned();
        }
        self
    }
    /// updates the response object only if `uri` is a valid URI.
    fn set_uri(&mut self, uri: &str) -> &mut ErrorResponse {
        if uri.is_ascii() && ErrorResponse::safe_uri(uri) {
            if uri.parse::<http::Uri>().is_ok() {
                self.uri = uri.to_owned();
            }
        }
        self
    }
    fn set_state(&mut self, state: &str) -> &mut ErrorResponse {
        self.state = state.as_bytes().to_vec();
        self
    }
}

#[derive(std::fmt::Debug)]
pub struct ResponseBody {
    map: serde_json::Map<String, serde_json::Value>,
}
impl ResponseBody {
    pub fn new() -> ResponseBody {
        ResponseBody { map : serde_json::Map::new() }
    }
    pub fn add(&mut self, key: &str, val: &str) -> &mut ResponseBody {
        if key.len() > 0 { 
            self.map.insert(key.to_owned(), serde_json::Value::from(val));
        }
        self
    }
    pub fn add_i64(&mut self, key: &str, val: i64) -> &mut ResponseBody {
         if key.len() > 0 { 
             self.map.insert(key.to_owned(), serde_json::Value::from(val));
         }
         self
    }
    pub fn add_f64(&mut self, key: &str, val: f64) -> &mut ResponseBody {
         if key.len() > 0 { 
             self.map.insert(key.to_owned(), serde_json::Value::from(val));
         }
         self
    }
    pub fn add_bool(&mut self, key: &str, val: bool) -> &mut ResponseBody {
         if key.len() > 0 { 
             self.map.insert(key.to_owned(), serde_json::Value::from(val));
         }
         self
    }
    pub fn de(&self) -> String {
        serde_json::to_string(&self.map).unwrap_or("".to_owned())
    }
}

impl AuthEndpoint {
    pub fn new(err: VertexError, host_port: String, sock: Option<SocketAddr>) -> AuthEndpoint {
        AuthEndpoint {err, host_port, sock}
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
            AuthEndpoint::new(err, String::from(host_addr), sock)
    }
    async fn process_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let (head, body) = req.into_parts();
        let uri = head.uri;
        let qp: QueryParams = QueryParams::from_uri(&uri);
        let headers: Headers = Headers::from(head.headers);
        println!("{}", headers);
        println!("\n");
        println!("has basic authz header {}", headers.authz_basic().unwrap_or("/have/nothing/"));
        println!("\n");
        let payload_size_max = match body.size_hint().upper() {
            Some(v) => v,
            None => RequestBody::PAYLOAD_SIZE_MAX + 1
        };
        if payload_size_max <= RequestBody::PAYLOAD_SIZE_MAX {
            if let Ok(body_bytes) = hyper::body::to_bytes(body).await {
                let rb = RequestBody::from_bytes(body_bytes);
                println!("RequestBody after serialization {:?}", rb);
            }
        }
        qp.resource();
        match uri.path() {
            "/authorize" => {
                let mut jr: ResponseBody = ResponseBody::new();
                jr.add("status", "Authorization code grant");
                qp.state().map(|val| { jr.add("state", val) });
                qp.nonce().map(|val| { jr.add("nonce", val) });
                let resp = Response::builder()
                    .status(StatusCode::OK)
                    .header("X-Indus-TxID", "1234")
                    .header("Content-Type", "application/json")
                    .header("Cache-Control", "no-store")
                    .header("Pragma", "no-cache")
                    .body(Body::from(jr.de()))
                    .unwrap();
                Ok(resp)
            },
            _ => {
                let mut jr: ResponseBody = ResponseBody::new();
                jr.add("error", "Authentication Failed");
                qp.state().map(|val| { jr.add("state", val) });
                qp.nonce().map(|val| { jr.add("nonce", val) });
                let resp = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .header("Cache-Control", "no-store")
                    .header("Pragma", "no-cache")
                    .body(Body::from(jr.de()))
                    .unwrap();
                Ok(resp)
            }
        }
    }

    pub async fn run(self: &AuthEndpoint) {
        let new_service = make_service_fn(|_conn: &AddrStream| async {
            Ok::<_, Infallible>(service_fn(AuthEndpoint::process_request))
        });
        println!("Listening on {}", self.sock.unwrap());
        let server = Server::bind(&self.sock.unwrap()).serve(new_service);
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
