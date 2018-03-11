#[macro_use]
extern crate combine;
extern crate itertools;
extern crate num;
use std::env;

mod util;
mod args;

fn run(argv: &Vec<String>) -> Result<(), args::CliError> {
    let args = args::parse(&argv)?;
    println!(
        "main:{:?}\nread: {:?}\nwrite: {:?}",
        args.main,
        args.from.map(args::parse_format),
        args.to.map(args::parse_format),
    );
    Ok(())
}

fn main() {
    let args = env::args().collect();
    if let Err(e) = run(&args) {
        println!("woops: {:?}", e);
    }
}
