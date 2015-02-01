#![feature(core)]
#![deny(warnings)]

#[macro_use] extern crate log;

extern crate iron;
extern crate docopt;
extern crate "rustc-serialize" as rustc_serialize;

extern crate shove;

mod args;
mod logging;

fn main() {
    let args = args::parse_args();

    logging::init_logging(&args.flag_log_level);
    let app = shove::api::new();

    iron::Iron::new(app).listen("localhost:4000").unwrap();
}
