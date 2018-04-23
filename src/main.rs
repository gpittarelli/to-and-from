#[macro_use]
extern crate combine;
extern crate csv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate petgraph;
extern crate serde_json;
use std::{env, io, collections::HashMap, fs::File, io::{BufReader, Write},
          iter::Iterator};

mod util;
mod args;
mod formats;

lazy_static! {
    static ref STDIN: io::Stdin = io::stdin();
    static ref STDOUT: io::Stdout = io::stdout();
}

struct Format {
    from: fn(Box<BufReader<io::Stdin>>) -> Box<formats::text::TextIR>,
}

type FormatsMap = HashMap<String, Format>;

fn load_formats() -> FormatsMap {
    let mut formats: FormatsMap = HashMap::new();

    formats.insert(
        "json".to_string(),
        Format {
            from: formats::text::json_to_ir,
        },
    );

    formats
}

fn run(argv: Vec<String>, formats: FormatsMap) -> Result<(), args::CliError> {
    let args = args::parse(argv.clone())?;
    formats::text::load();

    let src: Box<formats::text::TextIR> =
        match args.from.map(args::parse_format).and_then(|p| p.path) {
            Some(p) => formats::text::json_to_ir(Box::new(BufReader::new(
                File::open(p)?,
            ))),
            None => formats::text::json_to_ir(Box::new(STDIN.lock())),
        };

    let dest: Box<Write> = Box::new(STDOUT.lock());

    formats::text::ir_to_csv(src, dest)?;

    Ok(())
}

fn main() {
    let formats = load_formats();
    let args = env::args().collect();

    if let Err(e) = run(args, formats) {
        println!("woops: {:?}", e);
    }
}
