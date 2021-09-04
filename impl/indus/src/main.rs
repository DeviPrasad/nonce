mod basic;
mod pleb;
use std::result::Result;
use pleb::logger;
use basic::http::node::vertex;

#[tokio::main]
async fn main() {
    if !logger::init_logging() {
        println!("Error - Log set up failed. Quitting. ");
        return;
    }
    log::info!("Starting Indus");
    vertex::run().await;
}
