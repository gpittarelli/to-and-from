use petgraph::Graph;
// use petgraph::dot::{Config, Dot};
use std::io::{BufRead, Write};
use std::ops::Index;
use std::collections::HashMap;

pub fn load() -> () {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    deps.extend_with_edges(&[
        (pg, fb),
        (pg, qc),
        (qc, rand),
        (rand, libc),
        (qc, libc),
    ]);
    //     println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
    //     deps
}

/// Intermediate representation for translating between simple data
/// file types

type DataRows = Iterator<Item = Box<HashMap<String, String>>>;

pub struct TextIR {
    rows: Box<DataRows>,
}

// trait TextFormat {
//     fn read(input: Box<BufReader<u8>) -> TextIR;
//     fn write<W: Write>(input: TextIR, BufWriter<W>);
// }

// struct Json {}
// impl TextFormat for Json {
//     fn read(input: Box<BufRead>) -> TextIR {}
//     fn write<W: Write>(input: TextIR, Box<BufWrite>);
// }

pub fn json_to_ir<'a, 'b, T: BufRead + 'static>(input: Box<T>) -> Box<TextIR> {
    let rows: Box<DataRows> = Box::new((*input).lines().map(|s| {
        // type inference lets us omit an explicit type signature (which
        // would be `HashMap<&str, &str>` in this example).
        let mut book_reviews = HashMap::new();

        // review some books.
        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        Box::new(book_reviews) // as Box<Index<String, Output = String>>
    }));

    Box::new(TextIR { rows: rows })
}

pub fn ir_to_csv(mut input: Box<TextIR>, mut output: Box<Write>) {
    input.rows.as_mut().for_each(|t| {
        write!(output, "{:?}\n", t);
    });
}
