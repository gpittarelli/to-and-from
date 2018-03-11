use std::str::FromStr;
use std::fmt;
use std::path::{Path, PathBuf};
use combine::{eof, many, optional, satisfy, ParseError, Parser, Stream, many1};
use util::basename;

#[derive(Debug, PartialEq)]
pub enum CliSection {
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

parser! {
    fn cli_section[I]()(I) -> CliSection
    where [I: Stream<Item = String>]
    {
        satisfy(|s: String| is_section(s.as_str()))
            .map(|s: String| s.parse::<CliSection>().unwrap())
    }
}

parser! {
    fn cli_section_filename[I]()(I) -> String
    where [I: Stream<Item = String>]
    {
        satisfy(|s: String| {
            match basename(s.as_str()) {
            Some(s) => is_section(s),
            None => false
        }})
    }
}

parser! {
    fn non_section_filename[I]()(I) -> String
    where [I: Stream<Item = String>]
    {
        satisfy(|s: String|
                match basename(s.as_str()) {
                    Some(s) => !is_section(s),
                    None => false
                })
    }
}

fn is_section(name: &str) -> bool {
    name.parse::<CliSection>().is_ok()
}

#[allow(dead_code)]
fn other_section(section: &CliSection) -> &CliSection {
    match section {
        &CliSection::From => &CliSection::To,
        &CliSection::To => &CliSection::From,
    }
}

/// Arguments after a "to" or "from" in the CLI; must have at least 1
/// (a filename or an extension)
type FormatArgs = Vec<String>;
parser! {
    fn format_args[I]()(I) -> FormatArgs
    where [I: Stream<Item = String>]
    {
        many1(satisfy(|s: String| !is_section(s.as_str())))
    }
}

// Options for to-and-from itself; not the to/from formatting options
type MainArgs = Vec<String>;
parser! {
    fn main_args[I]()(I) -> MainArgs
    where [I: Stream<Item = String>]
    {
        many(satisfy(|s: String| !is_section(s.as_str())))
    }
}

type FormatSection = (CliSection, FormatArgs);
parser! {
    fn format_section[I]()(I) -> FormatSection
    where [I: Stream<Item = String>]
    {
        (cli_section(), format_args())
    }
}

#[derive(Debug)]
pub struct CliArgs {
    pub main: MainArgs,
    pub from: Option<FormatArgs>,
    pub to: Option<FormatArgs>,
}

// informal grammar for the CLI:
//  to = "to"
//  from = "to"
//  to-args = "format" format-args | "filename" format-args
//  to-section = to to-args
//  from-args = "format" format-args | "filename" format-args
//  from-section = from from-args
//  bin-args = "--help"  # etc.
//  main-args = to-section (from-section)? | from-section to-section
//  path-main-args = "/path/to/to" to-args (from-section)?
//                   | "/path/to/from" from-args to-section
//  argv = "/path/to/to-and-from" bin-args main-args
//         | "/path/to/to-and-from" from-args "to" to-args
//         | "/path/to/to-and-from" to-args "from" from-args
//         | path-main-args
parser! {
    fn cli_args[I]()(I) -> CliArgs
    where [I: Stream<Item = String>]
    {
        (
            choice!(
                (
                    non_section_filename(),
                    main_args(),
                    optional(format_section()),
                    optional(format_section())
                ).map(|(_, m, a, b)| {
                    let mut from = None;
                    let mut to = None;

                    if let Some((t, a2)) = a {
                        match t {
                             CliSection::From => from = Some(a2),
                             CliSection::To => to = Some(a2),
                        }
                    }

                    if let Some((t, b2)) = b {
                        match t {
                             CliSection::From => from = Some(b2),
                             CliSection::To => to = Some(b2),
                        }
                    }

                    CliArgs {
                        main: m,
                        from: from,
                        to: to
                    }
                })
                // TODO: The rest of the CLI types
            ),
            eof()
        ).map(|(t, _)| t)
    }
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

impl<'a> From<ParseError<&'a [String]>> for CliError {
    fn from(error: ParseError<&'a [String]>) -> Self {
        println!("{:?}", error);
        CliError {
            message: "hi".to_string(),
        }
    }
}

/// Parse a command line (eg, C's argv)
pub fn parse(args: &Vec<String>) -> Result<CliArgs, CliError> {
    let (parsed, _) = cli_args().parse(args.as_slice())?;
    Ok(parsed)
}

#[derive(Debug)]
pub struct FileDescription {
    // If not present: stdio/stdout
    path: Option<PathBuf>,
    format: String,
}

/// Parse an individual file format (either a from or a to section)
pub fn parse_format(args: FormatArgs) -> FileDescription {
    println!("convert {:?}", args);

    let first = &args[0];
    let path = if first.contains(".") {
        Some(Path::new(&first))
    } else {
        None
    };

    let format = path.and_then(|p| p.extension())
        .and_then(|os| os.to_str())
        .map(|s| s.to_string())
        .unwrap_or(first.to_string());

    FileDescription {
        path: path.map(|p| p.to_path_buf()),
        format: format,
    }
}
