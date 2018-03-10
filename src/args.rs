// informal grammar for the CLI:
//  to-args = "format" format-args | "filename" format-args
//  from-args = "format" format-args | "filename" format-args
//  bin-args = "--help"  # none specified yet, but
//  main-args = "to" to-args ("from" from-args)? | "from" from-args "to" to-args
//  path-main-args = "/path/to/to" to-args ("from" from-args)? | "/path/to/from" from-args "to" to-args
//  argv = "/path/to/to-and-from" bin-args main-args
//         | "/path/to/to-and-from" from-args "to" to-args
//         | "/path/to/to-and-from" to-args "from" from-args
//         | path-main-args

use std::str::FromStr;
use std::fmt;
use itertools::Itertools;
use combine::{token, Parser, Stream};
use util::skip;

#[derive(Debug)]
enum CliSection {
    From,
    To,
}

impl FromStr for CliSection {
    type Err = CliError;

    fn from_str(s: &str) -> Result<CliSection, CliError> {
        match s {
            "from" => Ok(CliSection::From),
            "to" => Ok(CliSection::To),
            _ => Err(CliError {
                message: format!("Unknown section type! {}", s),
            }),
        }
    }
}

fn is_section(name: &str) -> bool {
    name.parse::<CliSection>().is_ok()
}

parser! {
    fn aborc[I]()(I) -> i32
    where [I: Stream<Item = String>]
    {
        token("123".to_string()).map(|_s: String| 123)
    }
}
fn other_section(section: &CliSection) -> &CliSection {
    match section {
        &CliSection::From => &CliSection::To,
        &CliSection::To => &CliSection::From,
    }
}

type GenericArgs = Vec<String>;

#[derive(Debug, PartialEq)]
pub struct Args {
    main: Option<GenericArgs>,
    from: Option<GenericArgs>,
    to: Option<GenericArgs>,
}

#[derive(Debug)]
struct Section {
    kind: CliSection,
    args: GenericArgs,
}

#[derive(Debug, Default)]
pub struct CliError {
    message: String,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLI Error: {}", self.message)
    }
}

fn read_section(
    args: &mut Iterator<Item = &String>,
) -> Result<Section, CliError> {
    let kind = args.next()
        .ok_or(CliError {
            message: "Expected to read a value".to_string(),
        })?
        .parse::<CliSection>()?;

    Ok(Section {
        kind: kind,
        args: args.peekable()
            .peeking_take_while(|s| !is_section(s))
            .cloned()
            .collect(),
    })
}

pub fn parse(args: &Vec<String>) -> Result<Args, CliError> {
    let mut step = args.iter();
    //     let filename = step.next().unwrap();

    let args_out = Args {
        main: None,
        from: None,
        to: None,
    };

    println!("step: {:?}", read_section(&mut step)?);

    //     let mut first_section;
    //     if !is_section(filename) {
    //         let first_args: Vec<String> = step.clone()
    //             .cloned()
    //             .take_while(|x| !is_section(x))
    //             .collect();
    skip(&mut step, 2u32);
    //         args.main = Some(first_args);
    //         first_section = step.next().unwrap().parse::<CliSection>();
    //     } else {
    //         first_section = step.next().unwrap().parse::<CliSection>();
    //     }

    //     println!("first: {:?}", first_section);

    Ok(args_out)
}
