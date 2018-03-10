#[macro_use]
extern crate combine;
extern crate itertools;
extern crate num;
use std::env;

mod util;
mod args;

fn main() {
    let args = env::args().collect();
    println!("parsed: {:?}\ninto: {:?}", args, args::parse(&args));
}
