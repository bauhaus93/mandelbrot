#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate generator;

use std::io::Write;
use log::Record;
use env_logger::{ Builder };
use env_logger::fmt::Formatter;

use generator::Generator;

fn main() {
    const SNAPSHOT_SIZE: [i32; 2]= [1920, 1080];
    const ENTROPY_THRESHOLD: f32 = 4.;
    init_custom_logger();

    match Generator::new(SNAPSHOT_SIZE, ENTROPY_THRESHOLD) {
        Ok(mut generator) => {
            match generator.run() {
                Ok(_) => {},
                Err(e) => {
                    error!("Generator::run: {}", e);
                }
            }
        },
        Err(e) => {
            error!("Generator::new: {}", e);
        }
    };
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter , record: &Record| {
        let time = chrono::Local::now();
        writeln!(buf, "[{} {:-5}] {}", time.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args()) 
    };
    Builder::from_default_env()
        .format(format)
        .init();
}
