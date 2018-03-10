#[macro_use]
extern crate combine;
extern crate itertools;
extern crate num;
use std::env;

mod util;
mod args;

fn main() {
    let args: Vec<String> = env::args().collect();
    //     let mut sections = args.split(|s| s == "to");

    let asdf: &[String] = &[
        "123".to_string(),
        //         "Pick".to_string(),
        //         "up".to_string(),
        //         "that".to_string(),
        //         "word!".to_string(),
    ];

    //     let hi = vec![1, 2, 3, 4, 5, 6];
    //     let mut it = hi.iter();
    //     let bye = it.by_ref()
    //         .peekable()
    //         .peeking_take_while(|x| **x < 3)
    //         .collect::<Vec<_>>();
    //     println!("{:?} {:?}", bye, it.next());

    //     let first_section: CliSection = basename(&args[0])
    //         .ok_or(CliError {
    //             message: "Bad first section".to_string(),
    //         })
    //         .and_then(|s| s.parse())
    //         .unwrap_or(CliSection::To);
    // .unwrap_or(args[1].parse().unwrap_or(CliSection::To));
    //     let second_section = other_section(&first_section);

    println!("parsed: {:?}\ninto: {:?}", args, args::parse(&args));
}
