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
    line: usize,
) -> () {
    importer.add_dummy_type("unknown");
    // PENDING: Make a check to replace obligatory fields with a dummy value
    // In single file it will receive all of the names
    // Iterate over every column
    for name in names.iter() {
        // handle types
        // let idx = statics::RECOGNIZED_NAMES
        //     .iter()
        //     .position(|&x| x == *name)
        //     .unwrap();
        // let typ = statics::RECOGNIZED_TYPES[idx];
        // let data: ParsedColumn = match typ {
        //     "str" => ParsedColumn::Text(caps[*name].to_string()),
        //     "int" => ParsedColumn::Numeric(caps[*name].parse::<u32>().expect("expected digit")),
        //     "dec" => ParsedColumn::Decimal(caps[*name].parse::<f64>().expect("expected decimal")),
        //     _ => ParsedColumn::Meta(caps[*name].to_string()),
        // };
        // PENDING: Improve conditionals, receive types somehow different
        let data: ParsedColumn = match *name {
            "n_id" | "e_id" | "e_from" | "e_to" | "e_weight" => {
                ParsedColumn::Numeric(caps[*name].parse::<u32>().expect("n_id is not numeric!"))
            }
            "n_type" | "e_type" => ParsedColumn::Text(caps[*name].to_string()),

            _ => ParsedColumn::Meta(caps[*name].to_string()),
        };

        importer.insert_data(name, data, line);
    }
}

fn import_vertex_file(
    path: &str,
    uuid_map: &mut HashMap<u32, Uuid>,
    format: &str,
) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();
    let has_n_type = true;

    // collect the column names as they get recognized
    let mut column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
    column_names.remove(0);
    // Check if mandatory columns are missing
    if !column_names.iter().any(|&x| x == "n_type") {
        let has_n_type = false;
    }

    // Create temporary collection to handle import
    let mut importer = NodeImporter::new(has_n_type);

    // Iterate over every line
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line_text = line.unwrap();

        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();

            // Parse the line into the relation table
            parse_line(&caps, &mut importer, column_names.to_vec(), i);
        } else {
            continue;
        }
    }
    importer.generate_id_map(uuid_map);
    println!("{:?}", uuid_map);
    importer.insert_to_db(uuid_map);
    Ok(true)
}

fn import_edge_file(path: &str, uuid_map: &HashMap<u32, Uuid>, format: &str) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();
    let has_e_type = true;
    // collect the column names as they get recognized
    let mut column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
    column_names.remove(0);

    // Check if mandatory columns are missing
    if !column_names.iter().any(|&x| x == "e_type") {
        let has_e_type = false;
    }

    // Create temporary collection to handle import
    let mut importer = EdgeImporter::new(has_e_type);

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line_text = line.unwrap();
        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();
            parse_line(&caps, &mut importer, column_names.to_vec(), i);
        } else {
            continue;
        }
    }
    // Create number of vertices as many as the variety of uuids
    importer.insert_to_db(uuid_map);
    Ok(true)
}

// fn import_unified(path: &str, uuid_map: &mut HashMap<u32, Uuid>, format: &str) -> io::Result<bool> {
//     println!("Unified Import...");
//     let file = File::open(path).expect("There was a problem reading the vertices file.");
//     let re = Regex::new(format).unwrap();
//     println!("Unified {:?}", re);
//     // collect the column names as they get recognized
//     let column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
//     let column_names = &column_names[1..];
//     // Create temporary collection to handle import
//     let mut n_importer = NodeImporter::new();
//     let mut e_importer = EdgeImporter::new();

//     for (i, line) in BufReader::new(file).lines().enumerate() {
//         let line_text = line.unwrap();
//         let line_string = String::from(line_text);
//         let caps: Captures;

//         // Handle lines that are empty or do not fit in the expression
//         if let Some(x) = re.captures(&line_string) {
//             caps = re.captures(&line_string).unwrap();
//             // Parse the line into the relation table
//             parse_line(&caps, &mut n_importer, column_names.to_vec(), i);
//             parse_line(&caps, &mut e_importer, column_names.to_vec(), i);
//         } else {
//             continue;
//         }
//     }
// Create the interperter for id to uuid
//     n_importer.generate_id_map(uuid_map);
//     // Insert nodes to db
//     n_importer.insert_to_db(uuid_map);
//     // Insert edges to db
//     n_importer.insert_to_db(uuid_map);

//     Ok(true)
// }

pub fn import_files(file_info: ImportType) -> Result<(), &'static str> {
    let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
    let e: bool;
    let v: bool;
    let u: bool;
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
            e = import_edge_file(x.file_path[1], &uuid_map, x.expression[1]).unwrap();
            u = false;
        }
        ImportType::Unified(x) => {
            e = false;
            v = false;
            // Convert name vector to hashmap
            let mut names: HashMap<&str, &str> = HashMap::with_capacity(x.col_names.len());
            // Handle Unified import
            // u = import_unified(x.file_path, &mut uuid_map, x.expression).unwrap();
            u = false;
        }
        _ => {
            e = false;
            v = false;
            u = false;
        }
    }
    // Error Checking
    if (e && v) || u {
        Ok(())
    } else {
        Err("There was an error with one or both of the files")
    }
}
