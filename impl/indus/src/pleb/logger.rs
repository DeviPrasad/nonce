use std::result::Result;
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ ConsoleAppender, Target },
        file::FileAppender,
    },
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    config::{Appender, Config, Root},
};

pub fn init_logging() -> bool {
    let mut lr = false;
    let level_info = log::LevelFilter::Info;
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build("log/trace.out");
    if logfile.is_ok() {
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile.unwrap())))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level_info)))
                    .build("stderr", Box::new(stderr)),
            ).build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(LevelFilter::Trace),);
        if config.is_ok() {
            let res = log4rs::init_config(config.unwrap());
            lr = res.is_ok();
        }
    }
    lr
}
