#[macro_use]
extern crate combine;
extern crate csv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate petgraph;
extern crate serde_json;
mod util;
mod args;
mod formats;
use std::{env, io, collections::HashMap, fs::File,
          io::{BufReader, BufWriter, Write}, iter::Iterator, path::PathBuf};
use formats::text::TextIR;
use args::CliError;

lazy_static! {
// TODO: Recover cross-platform support; would do via trait
// objects but we need them to be Sized
//    static ref STDIN: io::Stdin = io::stdin();
    static ref STDOUT: io::Stdout = io::stdout();
}

#[derive(Debug)]
struct Format {
    from: fn(Box<BufReader<File>>) -> Box<TextIR>,
    to: fn(Box<TextIR>, Box<Write>) -> Result<(), CliError>,
}

type FormatsMap = HashMap<String, Format>;

fn load_formats() -> FormatsMap {
    let mut formats = FormatsMap::new();

    formats.insert(
        "json".to_string(),
        Format {
            from: formats::text::json_to_ir,
            to: formats::text::ir_to_json,
        },
    );

    formats.insert(
        "csv".to_string(),
        Format {
            from: formats::text::csv_to_ir,
            to: formats::text::ir_to_csv,
        },
    );

    formats
}

fn open(p: PathBuf) -> Result<Box<BufReader<File>>, io::Error> {
    Ok(Box::new(BufReader::new(File::open(p)?)))
}

fn open_out(p: PathBuf) -> Result<Box<Write>, io::Error> {
    Ok(Box::new(BufWriter::new(File::create(p)?)))
}

fn run(argv: Vec<String>, formats: FormatsMap) -> Result<(), args::CliError> {
    let args = args::parse(argv)?;

    if args.from.is_none() {
        return Err(CliError::from_error());
    }

    let from = args::parse_format(args.from.unwrap());
    let to = args::parse_format(args.to.unwrap());

    let input = open(from.path.clone().unwrap_or(PathBuf::from("/dev/stdin")))?;
    let dest = to.path
        .clone()
        .map(open_out)
        .unwrap_or(Ok(Box::new(BufWriter::new(STDOUT.lock()))))?;

    let data = (formats
        .get(&from.format)
        .ok_or(CliError::unknown_format(&from))?
        .from)(input);

    (formats
        .get(&to.format)
        .ok_or(CliError::unknown_format(&to))?
        .to)(data, dest)?;

    Ok(())
}

fn main() {
    let formats = load_formats();
    let args = env::args().collect();

    if let Err(e) = run(args, formats) {
        eprintln!("{}", e);
    }
}
