//use log::{debug, error, log_enabled, info, Level};
//use env_logger::{Builder, Target};

pub fn init_logging() -> bool {
    env_logger::init();
    //let mut builder = Builder::from_default_env();
    //builder.target(Target::Stdout);
    true
}
