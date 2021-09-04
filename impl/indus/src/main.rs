use std::result::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
mod basic;

fn main() -> Result<(), std::io::Error>{
    println!("Indus");
    basic::http::node::vertex::run();
    Ok(())
}
