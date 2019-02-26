#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate mandelbrot;

use std::io::Write;
use log::Record;
use env_logger::{ Builder };
use env_logger::fmt::Formatter;

use mandelbrot::application::Application;

fn main() {
    const WINDOW_SIZE: [i32; 2]= [400, 400];
    const UPDATE_FREQUENCY: i32 = 30;
    init_custom_logger();

    match Application::new(WINDOW_SIZE, UPDATE_FREQUENCY) {
        Ok(mut app) => {
            app.run();        
        },
        Err(e) => {
            error!("{}", e);
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
