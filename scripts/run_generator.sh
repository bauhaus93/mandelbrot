#!/bin/sh

log_level="debug"
log_string="$log_level"
RUST_LOG=$log_string cargo run --release --bin generator_app
