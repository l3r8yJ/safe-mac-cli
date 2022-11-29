#[deny(warnings)]
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[ctor::ctor]
fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();
}
