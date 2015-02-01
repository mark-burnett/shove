#![feature(collections)]
#![deny(warnings)]

#[macro_use] extern crate rustless;
#[macro_use] extern crate log;

extern crate iron;
extern crate "rustc-serialize" as serialize;

pub mod api;
pub mod topics;
