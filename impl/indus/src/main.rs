mod basic;
mod oidc10;
mod pleb;
use pleb::logger;
use basic::http::node::vertex;

#[tokio::main]
async fn main() {
    if !logger::init_logging() {
        println!("Error - Log set up failed. Quitting. ");
        std::process::exit(1)
    }
    log::info!("Starting Indus");
    vertex::run().await;
}
