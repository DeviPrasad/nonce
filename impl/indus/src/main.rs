mod basic;
mod pleb;
use pleb::logger;
use basic::http::auth_endpoint::AuthEndpoint;

#[tokio::main]
async fn main() {
    if !logger::init_logging() {
        println!("Error - Logger init failed. Quitting. ");
        std::process::exit(1);
    }
    log::info!("Starting Indus");
    AuthEndpoint::start().await;
}
