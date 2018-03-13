use petgraph::Graph;
// use petgraph::dot::{Config, Dot};
use std::io::{BufRead, Write};
use std::ops::Index;
use std::collections::HashMap;
use serde_json;
use serde_json::{Error, Value};
use std::iter;
use std::slice;
use std;
use std::vec;
use csv::Writer;

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

fn join_path(base: String, child: String) -> String {
    if base.len() == 0 {
        child
    } else {
        let mut s = base.clone();
        s.push_str(".");
        s.push_str(&child);
        s
    }
}

fn walk_json(
    path: String,
    val: Value,
) -> Box<Iterator<Item = (String, String)>> {
    match val {
        Value::Null => panic!("WTF NULL"),
        Value::Bool(b) => panic!("WTF bool"),
        Value::Number(n) => panic!("WTF num"),
        Value::String(s) => panic!("WTF string"),
        Value::Array(a) => {
            //             let i = 0;
            Box::new(a.into_iter().flat_map(move |v| {
                //                 i = i + 1;
                match v {
                    Value::Null => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                    Value::Bool(ref b) => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                    Value::Number(ref n) => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                    Value::String(ref s) => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                    Value::Array(ref a2) => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                    Value::Object(ref o) => vec![
                        (format!("{:?}a", path), "null".to_string()),
                    ].into_iter(),
                }
            })) as Box<Iterator<Item = (String, String)>>
        }
        Value::Object(o) => {
            //            panic!("WTF object");
            Box::new(o.into_iter().flat_map(move |(k, v)| {
                match v {
                    Value::Null => vec![
                        (join_path(path.clone(), k), "null".to_string()),
                    ].into_iter(),
                    Value::Bool(b) => vec![
                        (
                            join_path(path.clone(), k),
                            (if b { "true" } else { "false" }).to_string(),
                        ),
                    ].into_iter(),
                    Value::Number(n) => vec![
                        (join_path(path.clone(), k), n.to_string()),
                    ].into_iter(),
                    Value::String(ref s) => vec![
                        (join_path(path.clone(), k), "null".to_string()),
                    ].into_iter(),
                    Value::Array(ref a2) => vec![
                        (join_path(path.clone(), k), "null".to_string()),
                    ].into_iter(),
                    Value::Object(ref o) => vec![
                        (join_path(path.clone(), k), "null".to_string()),
                    ].into_iter(),
                }
            })) as Box<Iterator<Item = (String, String)>>
        }
    }
}

/// Flattens a serde_json Value into a hashmap from object path -> value
pub fn json_to_row(input: Value) -> Box<HashMap<String, String>> {
    let mut book_reviews = HashMap::new();

    for (k, v) in walk_json("".to_string(), input) {
        book_reviews.insert(k, v);
    }

    Box::new(book_reviews)
}

pub fn json_to_ir<'a, 'b, T: BufRead + 'static>(input: Box<T>) -> Box<TextIR> {
    let rows: Box<DataRows> = Box::new((*input).lines().map(|s| {
        let v: Value = serde_json::from_str(s.unwrap().as_str()).unwrap();
        json_to_row(v)
    }));

    Box::new(TextIR { rows: rows })
}

pub fn ir_to_csv(mut input: Box<TextIR>, mut output: Box<Write>) {
    let mut csv_output = Writer::from_writer(output);
    input
        .rows
        .as_mut()
        .for_each(|t: Box<HashMap<String, String>>| {
            let values: Vec<&String> = (*t).values().collect();
            csv_output.encode(values);
        });
}
