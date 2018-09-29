// use uuid::Uuid;
use io::importer::{initialize_spatial, EdgeImporter, NodeImporter, UnifiedImporter};
use io::pattern::{DualPattern, ImportType, SinglePattern};
use regex::{Captures, Regex};
use statics;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::{fs::File, io};
use uuid::Uuid;
// TODO: Receive column declarations about which part is what (ex. source,target, label, type, weight)

static RECOGNIZED_NAMES: [&str; 9] = [
    "n_id", "n_label", "n_type", "e_id", "e_from", "e_to", "e_type", "e_label", "e_weight"
];

static RECOGNIZED_TYPES: [&str; 9] = [
    "int", "meta", "str", "int", "int", "int", "str", "str", "dec"
];

enum Importer {
    NodeImporter,
    EdgeImporter,
}

#[derive(Debug)]
pub enum ParsedColumn {
    Numeric(u32),
    Text(String),
    Decimal(f64),
    Meta(String),
}

// Is repeated for every line
fn parse_line(
    caps: &Captures,
    relation_table: &mut NodeImporter,
    names: Vec<&str>, // This is what differs between single and unified
) -> () {
    // In single file it will receive all of the names
    // Iterate over every column
    for name in names.iter() {
        // handle types
        let idx = RECOGNIZED_NAMES.iter().position(|&x| x == *name).unwrap();
        let typ = RECOGNIZED_TYPES[idx];

        let data: ParsedColumn = match typ {
            "str" => ParsedColumn::Text(caps[*name].to_string()),
            "int" => ParsedColumn::Numeric(caps[*name].parse::<u32>().expect("expected digit")),
            "dec" => ParsedColumn::Decimal(caps[*name].parse::<f64>().expect("expected decimal")),
            _ => ParsedColumn::Meta(caps[*name].to_string()),
        };

        relation_table.insert_data(name, data);
    }
}

fn process_e_line(line: String, caps: &Captures, relation_table: &mut EdgeImporter) -> () {
    let from_to = (
        caps["id"].parse::<u32>().expect("expected digit"),
        caps["from"].parse::<u32>().expect("expected digit"),
        caps["to"].parse::<u32>().expect("expected digit"),
        &caps["label"],
        &caps["type"],
        caps["weight"].parse::<u8>().expect("expected digit"),
    );
    relation_table.update(from_to);
}

fn process_u_line(line: String, caps: &Captures, relation_table: &mut UnifiedImporter) -> () {
    let parsed = (
        caps["id"].parse::<u32>().expect("expected digit"),
        caps["from"].parse::<u32>().expect("expected digit"),
        caps["to"].parse::<u32>().expect("expected digit"),
        &caps["label"],
        &caps["type"],
        caps["weight"].parse::<u8>().expect("expected digit"),
    );
    relation_table.update(parsed);
}

fn import_vertices(
    path: &str,
    uuid_map: &mut HashMap<u32, Uuid>,
    format: &str,
) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();

    // collect the column names as they get recognized
    let column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
    let column_names = &column_names[1..];

    // Create temporary collection to handle import
    let mut relation_table = NodeImporter::new();

    // Iterate over every line
    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();

        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();
            // Parse the line into the relation table
            parse_line(&caps, &mut relation_table, column_names.to_vec());
        } else {
            continue;
        }
    }
    *uuid_map = relation_table.generate_id_map().unwrap();
    // relation_table.generate_type_map();
    relation_table.create_vertices();

    initialize_spatial();

    Ok(true)
}

fn import_edges(
    path: &str,
    uuid_map: &HashMap<u32, Uuid>,
    format: &str,
) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the edges file.");
    let from_to: (u32, u32);
    let t = 0;
    // Regular expression pattern for edge list
    let re = Regex::new(format).unwrap();
    // Create temporary collection to handle import
    let mut relation_table = EdgeImporter::new();

    println!("Parsing file {}", path);

    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();
        let line_string = &String::from(line_text);
        let caps;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(line_string) {
            caps = re.captures(line_string).unwrap();
            process_e_line(line_string.to_owned(), &caps, &mut relation_table);
        } else {
            continue;
        }
    }
    // Create number of vertices as many as the variety of uuids
    relation_table.create_edges(&uuid_map);
    Ok(true)
}

fn import_unified(
    path: &str,
    uuid_map: &HashMap<u32, Uuid>,
    format: &str,
    names: HashMap<&str, &str>,
) -> io::Result<bool> {
    println!("Unified Import...");
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();
    println!("Unified {:?}", re);

    // Create temporary collection to handle import
    let mut relation_table = UnifiedImporter::new();

    println!("Parsing file {}", path);
    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();

        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();

            // Parse the line into the relation table

            process_u_line(line_string.to_owned(), &caps, &mut relation_table);
        } else {
            continue;
        }
    }

    Ok(true)
}

pub fn import_files(file_info: ImportType) -> Result<(), &'static str> {
    let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
    let e: bool;
    let v: bool;

    // Handle the possibility of not setting a node filepath
    match file_info {
        ImportType::Dual(x) => {

            // Convert name vectors to hashmaps
            let mut n_names: HashMap<&str, &str> = HashMap::with_capacity(x.n_names.len());
            let mut e_names: HashMap<&str, &str> = HashMap::with_capacity(x.e_names.len());
            x.n_names.iter().map(|x| n_names.insert(x.0, x.1));
            x.e_names.iter().map(|x| e_names.insert(x.0, x.1));
            // Call separate importers
            v = import_vertices(x.file_path[0], &mut uuid_map, x.expression[0]).unwrap();
            e = import_edges(x.file_path[1], &uuid_map, x.expression[1]).unwrap();
        }
        ImportType::Unified(x) => {
            // Convert name vector to hashmap
            let mut names: HashMap<&str, &str> = HashMap::with_capacity(x.col_names.len());
            // Handle Unified import
            v = import_unified(x.file_path, &mut uuid_map, x.expression, names).unwrap();
            e = v;
        }
        _ => {
            e = false;
            v = false;
        }
    }
    // Error Checking
    if e && v {
        Ok(())
    } else {
        Err("There was an error with one or both of the files")
    }
}

pub enum PatternFormat<'a> {
    Dual([&'a str; 2]),
    Unified([&'a str; 1]),
}
