#[macro_use]
extern crate combine;
extern crate csv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate petgraph;
extern crate serde_json;
use std::env;
use std::iter::Iterator;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

mod util;
mod args;
mod formats;

lazy_static! {
    static ref STDIN: io::Stdin = io::stdin();
    static ref STDOUT: io::Stdout = io::stdout();
}

fn run(argv: Vec<String>) -> Result<(), args::CliError> {
    let args = args::parse(&argv)?;
    formats::text::load();

    println!("HI! {:?} {:?}", args, args);

    let src: Box<formats::text::TextIR> =
        match args.from.map(args::parse_format).and_then(|p| p.path) {
            Some(p) => formats::text::json_to_ir(Box::new(BufReader::new(
                File::open(p)?,
            ))),
            None => formats::text::json_to_ir(Box::new(STDIN.lock())),
        };

    let mut dest: Box<Write> = Box::new(STDOUT.lock());

    formats::text::ir_to_csv(src, dest);

    Ok(())
}

fn main() {
    let args = env::args().collect();

    if let Err(e) = run(args) {
        println!("woops: {:?}", e);
    }
}
