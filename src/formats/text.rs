use args;
use csv;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufRead, Write};

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
        // We should only ever recurse into arrays or objects:
        Value::Null => panic!("walk_json: hit a primitive null"),
        Value::Bool(_) => panic!("walk_json: hit a primitive bool"),
        Value::Number(_) => panic!("walk_json: hit a primitive number"),
        Value::String(_) => panic!("walk_json: hit a primitive string"),

        Value::Array(a) => {
            Box::new(a.into_iter().enumerate().flat_map(move |(i, v)| {
                let new_path = format!("{:}[{:?}]", path, i);
                match v {
                    Value::Null => Box::new(
                        vec![(new_path, "null".to_string())].into_iter(),
                    ),
                    Value::Bool(ref b) => {
                        Box::new(vec![(new_path, b.to_string())].into_iter())
                    }
                    Value::Number(ref n) => {
                        Box::new(vec![(new_path, n.to_string())].into_iter())
                    }
                    Value::String(ref s) => {
                        Box::new(vec![(new_path, s.to_string())].into_iter())
                    }
                    Value::Array(ref _a2) => Box::new(
                        vec![(new_path, "arr".to_string())].into_iter(),
                    ),
                    Value::Object(_) => walk_json(new_path, v),
                }
            })) as Box<Iterator<Item = (String, String)>>
        }

        Value::Object(o) => Box::new(o.into_iter().flat_map(move |(k, v)| {
            match v {
                Value::Null => Box::new(
                    vec![(join_path(path.clone(), k), "null".to_string())]
                        .into_iter(),
                ),
                Value::Bool(b) => Box::new(
                    vec![(
                        join_path(path.clone(), k),
                        (if b { "true" } else { "false" }).to_string(),
                    )].into_iter(),
                ),
                Value::Number(n) => Box::new(
                    vec![(join_path(path.clone(), k), n.to_string())]
                        .into_iter(),
                ),
                Value::String(ref s) => Box::new(
                    vec![(join_path(path.clone(), k), s.to_string())]
                        .into_iter(),
                ),
                Value::Array(_) => walk_json(join_path(path.clone(), k), v),
                Value::Object(ref _o) => Box::new(
                    vec![(join_path(path.clone(), k), "obj".to_string())]
                        .into_iter(),
                ),
            }
        }))
            as Box<Iterator<Item = (String, String)>>,
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

pub fn json_to_ir<T: BufRead + 'static>(input: Box<T>) -> Box<TextIR> {
    let rows: Box<DataRows> = Box::new((*input).lines().map(|s| {
        let v: Value = serde_json::from_str(s.unwrap().as_str()).unwrap();
        json_to_row(v)
    }));

    Box::new(TextIR { rows: rows })
}

pub fn ir_to_json(
    input: Box<TextIR>,
    mut output: Box<Write>,
) -> Result<(), args::CliError> {
    for t in input.rows {
        let mut o = serde_json::Map::new();
        for (k, v) in t.iter() {
            o.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
        output.write(serde_json::to_string(&o)?.as_bytes())?;
        output.write(b"\n")?;
    }
    output.flush()?;

    Ok(())
}

pub fn csv_to_ir<T: BufRead + 'static>(input: Box<T>) -> Box<TextIR> {
    //    let a = Box::new(csv::Reader::from_reader(*input));
    let rows = Box::new((*input).lines().map(|_s| {
        let r = HashMap::new();
        //let v: Value = serde_json::from_str(s.unwrap().as_str()).unwrap();
        Box::new(r)
    }));

    Box::new(TextIR { rows: rows })
}

pub fn ir_to_csv(
    input: Box<TextIR>,
    output: Box<Write>,
) -> Result<(), args::CliError> {
    let mut csv_output = csv::Writer::from_writer(output);

    let mut rows = input.rows.peekable();

    let mut keys: Vec<String> = vec![];
    {
        let x = rows.peek().unwrap();
        for k in x.keys() {
            keys.push(k.to_string());
        }
    }

    csv_output.serialize(keys.clone())?;

    for t in rows {
        //         if keys.is_none() {
        //             let k = t.clone();
        //             keys = Some((*k).keys().collect::<Vec<&String>>());
        //         }
        let row = *t;
        let out = keys.iter().map(|k| row.get(k)).collect::<Vec<_>>();
        csv_output.serialize(out)?;
    }

    Ok(())
}
