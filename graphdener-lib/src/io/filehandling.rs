// use uuid::Uuid;
use io::importer::Importer;
use io::importer::{EdgeImporter, NodeImporter};
use io::pattern::InitPattern;
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
    has_type: bool,
) -> () {
    //
    let mut clean_name: &str;

    // Iterate over every column
    for name in names.iter() {
        // Clean the name if it contains numbers because of duplicates in order to do the match
        if name[name.len() - 1..].parse::<u8>().is_ok() {
            clean_name = &name[..name.len() - 1];
        } else {
            clean_name = name;
        }

        let data: ParsedColumn = match clean_name {
            "n_id" | "e_id" | "e_from" | "e_to" | "e_weight" => {
                ParsedColumn::Numeric(caps[*name].parse::<u32>().expect("n_id is not numeric!"))
            }
            "n_type" | "e_type" => ParsedColumn::Text(caps[*name].to_string()),

            _ => ParsedColumn::Meta(caps[*name].to_string()),
        };

        importer.insert_data(clean_name, data, line);
    }

    if !has_type {
        if let Importer::NodeImporter = importer.get_importer_type() {
            importer.add_dummy_type(1, "unknown");
        } else if let Importer::EdgeImporter = importer.get_importer_type() {
            importer.add_dummy_type(line as u32, "unknown");
        }
    }
}

fn import_vertex_file(
    path: &str,
    uuid_map: &mut HashMap<u32, Uuid>,
    format: &str,
) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();
    let mut has_n_type = true;

    // collect the column names as they get recognized
    let mut column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
    column_names.remove(0);
    // Check if mandatory columns are missing
    if !column_names.iter().any(|&x| x == "n_type") {
        has_n_type = false;
    }

    // Create temporary collection to handle import
    let mut importer = NodeImporter::new();

    // Iterate over every line
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line_text = line.unwrap_or("".to_string());

        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();

            // Parse the line into the relation table
            parse_line(&caps, &mut importer, column_names.to_vec(), i, has_n_type);
        } else {
            continue;
        }
    }
    importer.generate_id_map(uuid_map);
    importer.insert_to_db(uuid_map);
    Ok(true)
}

// TODO: Unify import_edge with import_vertex
fn import_edge_file(path: &str, uuid_map: &HashMap<u32, Uuid>, format: &str) -> io::Result<bool> {
    let file = File::open(path).expect("There was a problem reading the vertices file.");
    let re = Regex::new(format).unwrap();
    let mut has_e_type = true;
    // collect the column names as they get recognized
    let mut column_names: Vec<&str> = re.capture_names().map(|x| x.unwrap_or("")).collect();
    column_names.remove(0);

    // Check if mandatory columns are missing
    if !column_names.iter().any(|&x| x == "e_type") {
        has_e_type = false;
    }
    // Create temporary collection to handle import
    let mut importer = EdgeImporter::new();

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line_text = line.unwrap_or("".to_string());
        let line_string = String::from(line_text);
        let caps: Captures;

        // Handle lines that are empty or do not fit in the expression
        if let Some(x) = re.captures(&line_string) {
            caps = re.captures(&line_string).unwrap();
            parse_line(&caps, &mut importer, column_names.to_vec(), i, has_e_type);
        } else {
            continue;
        }
    }
    // Create number of vertices as many as the variety of uuids
    importer.insert_to_db(uuid_map);
    Ok(true)
}

pub fn import_files(file_info: InitPattern) -> Result<(), &'static str> {
    let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
    let e: bool;
    let v: bool;

    // Convert name vectors to hashmaps
    let mut n_names: HashMap<&str, &str> = HashMap::with_capacity(file_info.n_names.len());
    let mut e_names: HashMap<&str, &str> = HashMap::with_capacity(file_info.e_names.len());
    file_info.n_names.iter().map(|x| n_names.insert(x.0, x.1));
    file_info.e_names.iter().map(|x| e_names.insert(x.0, x.1));
    // Call separate importers
    v = import_vertex_file(
        file_info.file_path[0],
        &mut uuid_map,
        file_info.expression[0],
    )
    .unwrap();
    e = import_edge_file(file_info.file_path[1], &uuid_map, file_info.expression[1]).unwrap();

    // Error Checking
    if (e && v) {
        Ok(())
    } else {
        Err("There was an error with one or both of the files")
    }
}
