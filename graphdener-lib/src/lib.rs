extern crate graphdenerdb;
extern crate rand;
extern crate regex;
extern crate rmp_rpc;
extern crate uuid;
extern crate arrayfire as af;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

mod alg;
pub mod commands;
mod io;
pub mod models;
mod statics;
mod traits;
