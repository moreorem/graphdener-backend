// use uuid::Uuid;
use io::importer::{initialize_spatial, EdgeImporter, NodeImporter, UnifiedImporter};
use regex::{Captures, Regex};
use statics;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::{fs::File, io};
use uuid::Uuid;

// TODO: Receive column declarations about which part is what (ex. source,target, label, type, weight)

enum Importer {
    NodeImporter,
    EdgeImporter,
}

fn process_n_line(line: String, caps: &Captures, relation_table: &mut NodeImporter) -> () {
    let id_label_type: (u32, &str, &str) = (
        caps["id"].parse::<u32>().expect("expected digit"),
        &caps["label"],
        &caps["type"],
    );
    relation_table.update(id_label_type);
}

// TODO: get caps keys from frontend

fn process_e_line(line: String, caps: &Captures, relation_table: &mut EdgeImporter) -> () {
    let from_to: (u32, u32, u32, &str, &str, u8) = (
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

    // Create temporary collection to handle import
    let mut relation_table = NodeImporter::new();

    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();

        let line_string = String::from(line_text);
        let caps: Captures;
        let id_label_type: (u32, &str, &str);

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();

            // Parse the line into the relation table
            process_n_line(line_string.to_owned(), &caps, &mut relation_table);
        } else {
            continue;
        }
    }

    *uuid_map = relation_table.generate_id_map().unwrap();

    relation_table.generate_type_map();
    relation_table.create_vertices();

    initialize_spatial();

    Ok(true)
}

fn import_edges(path: &str, uuid_map: &HashMap<u32, Uuid>, format: &str) -> io::Result<bool> {
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

fn import_unified(path: &str, uuid_map: &HashMap<u32, Uuid>, format: &str) -> io::Result<bool> {
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
        let id_label_type: (u32, &str, &str);

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

pub fn import_files(
    vert_path: &str,
    edge_path: &str,
    format: PatternFormat,
) -> Result<(), &'static str> {
    let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
    let e: bool;
    let v: bool;

    // Handle the possibility of not setting a node filepath
    match format {
        PatternFormat::Dual(f) => {
            println!("{}", f[0]);
            v = import_vertices(vert_path, &mut uuid_map, f[0]).unwrap();
            e = import_edges(edge_path, &uuid_map, f[1]).unwrap();
        }
        PatternFormat::Unified(f) => {
            // Handle Unified import
            v = import_unified(vert_path, &mut uuid_map, f[0]).unwrap();
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

// fn file_handle<T>(path: &str, format: &str, importer: T) -> () {
//     let file = File::open(path).expect("There was a problem reading the vertices file.");
//     let re = Regex::new(format).unwrap();
//     println!("Unified {:?}", re);

//     // Create temporary collection to handle import
//     let mut relation_table = importer::new();
// }
