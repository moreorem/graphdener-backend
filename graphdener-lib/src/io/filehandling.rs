// use uuid::Uuid;
use io::importer::Importer;
use io::importer::{EdgeImporter, NodeImporter, UnifiedImporter};
use io::pattern::ImportType;
use regex::{Captures, Regex};
use statics;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::{fs::File, io};
use traits::Import;
use uuid::Uuid;

// TODO: Receive column declarations about which part is what (ex. source,target, label, type, weight)

#[derive(Debug)]
pub enum ParsedColumn {
    Numeric(u32),
    Text(String),
    Decimal(f64),
    Meta(String),
}

// Is repeated for every line
fn parse_line<T: Import>(
    caps: &Captures,
    importer: &mut T,
    names: Vec<&str>, // This is what differs between single and unified
) -> () {
    // In single file it will receive all of the names
    // Iterate over every column
    for name in names.iter() {
        // handle types
        let idx = statics::RECOGNIZED_NAMES
            .iter()
            .position(|&x| x == *name)
            .unwrap();
        let typ = statics::RECOGNIZED_TYPES[idx];

        let data: ParsedColumn = match typ {
            "str" => ParsedColumn::Text(caps[*name].to_string()),
            "int" => ParsedColumn::Numeric(caps[*name].parse::<u32>().expect("expected digit")),
            "dec" => ParsedColumn::Decimal(caps[*name].parse::<f64>().expect("expected decimal")),
            _ => ParsedColumn::Meta(caps[*name].to_string()),
        };

        importer.insert_data(name, data);
    }
}

fn import_vertex_file(
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
    let mut importer = NodeImporter::new();

    // Iterate over every line
    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();

        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();
            // Parse the line into the relation table
            parse_line(&caps, &mut importer, column_names.to_vec());
        } else {
            continue;
        }
    }
    *uuid_map = importer.generate_id_map().unwrap();
    importer.create_vertices();
    // initialize_spatial();
    Ok(true)
}

fn import_edge_file(
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
    let mut importer = EdgeImporter::new();

    for line in BufReader::new(file).lines() {
        let line_text = line.unwrap();
        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();
            parse_line(&caps, &mut importer, column_names.to_vec());
        } else {
            continue;
        }
    }
    // Create number of vertices as many as the variety of uuids
    // importer.create_edges(&uuid_map);
    Ok(true)
}

// fn import_unified(
//     path: &str,
//     uuid_map: &HashMap<u32, Uuid>,
//     format: &str,
//     names: HashMap<&str, &str>,
// ) -> io::Result<bool> {
//     println!("Unified Import...");
//     let file = File::open(path).expect("There was a problem reading the vertices file.");
//     let re = Regex::new(format).unwrap();
//     println!("Unified {:?}", re);

//     // Create temporary collection to handle import
//     let mut relation_table = UnifiedImporter::new();

//     println!("Parsing file {}", path);
//     for line in BufReader::new(file).lines() {
//         let line_text = line.unwrap();

//         let line_string = String::from(line_text);
//         let caps: Captures;

//         // Handle lines that are empty or do not fit in the expression
//         if let Some(x) = re.captures(&line_string) {
//             caps = re.captures(&line_string).unwrap();

//             // Parse the line into the relation table

//             process_u_line(line_string.to_owned(), &caps, &mut relation_table);
//         } else {
//             continue;
//         }
//     }

//     Ok(true)
// }

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
            v = import_vertex_file(x.file_path[0], &mut uuid_map, x.expression[0]).unwrap();
            // e = import_edges(x.file_path[1], &uuid_map, x.expression[1]).unwrap();
            e = true;
        }
        ImportType::Unified(x) => {
            // Convert name vector to hashmap
            let mut names: HashMap<&str, &str> = HashMap::with_capacity(x.col_names.len());
            // Handle Unified import
            v = true; //import_unified(x.file_path, &mut uuid_map, x.expression, names).unwrap();
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
