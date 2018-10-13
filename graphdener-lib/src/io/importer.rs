use super::super::commands::database;
use super::filehandling::ParsedColumn;
use indradb::util;
use std::collections::HashMap;
use traits::Import;
use uuid::Uuid;

pub enum Importer {
    NodeImporter,
    EdgeImporter,
}

pub struct NodeImporter {
    node_list: Vec<u32>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
    current_id: u32,
}

// Because there is no edge id usually we create it using line number
pub struct EdgeImporter {
    edge_list: Vec<[u32; 2]>,
    pub type_list: Vec<(usize, String)>,
    pub meta_list: Vec<(usize, HashMap<String, String>)>,
    temp_fromto: [Option<u32>; 2],
}

// Import trait implementation for edge importer
impl Import for EdgeImporter {
    fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize) {
        let mut meta = String::from("");
        match name {
            "e_id" => {
                if let ParsedColumn::Numeric(x) = data {
                    // Dummy value, we probably wont use e_id because we generate our own
                    // self.line_number.push(x)
                } else {
                    panic!("unknown id type");
                }
            }
            "e_from" => {
                if let ParsedColumn::Numeric(x) = data {
                    self.temp_fromto[0] = Some(x);
                } else {
                    panic!("unknown source type");
                }
            }
            "e_to" => {
                if let ParsedColumn::Numeric(x) = data {
                    self.temp_fromto[1] = Some(x);
                } else {
                    panic!("unknown target type");
                }
            }
            "e_type" => {
                if let ParsedColumn::Text(x) = data {
                    self.type_list.push((line, x));

                } else {
                    panic!("unknown type type");
                }
            }
            _ => {
                if let ParsedColumn::Meta(x) = data {
                    meta = x.to_owned()
                }
            }
        };
        // If meta variable has data, pass it to the metadata list
        if !meta.is_empty() {
            let mut meta_map = HashMap::new();
            meta_map.insert(name.to_string(), meta);
            self.meta_list.push((line, meta_map));
        }
        // If temporary from-to line is filled empty the contents into the edgelist
        if let [Some(x), Some(y)] = self.temp_fromto {
            self.edge_list.push([x, y]);
            self.temp_fromto = [None, None];
        }
    }

    fn add_dummy_type(&mut self, id: u32, typ: &str) {
        self.type_list.push((id as usize, typ.to_string()));
    }

    fn get_importer_type(&self) -> Importer {
        Importer::EdgeImporter
    }
}

impl EdgeImporter {
    pub fn new() -> EdgeImporter {
        EdgeImporter {
            edge_list: Vec::new(),
            type_list: Vec::new(),
            meta_list: Vec::new(),
            temp_fromto: [None, None],
        }
    }

    // Sends the edge data to database
    pub fn insert_to_db(&self, uuid_map: &HashMap<u32, Uuid>) -> () {
        println!("Storing edges to database...");

        for (i, id) in self.edge_list.iter().enumerate() {
            let (from, t, to) = (
                *uuid_map.get(&id[0]).unwrap(),
                self.type_list[i].1.to_owned(),
                *uuid_map.get(&id[1]).unwrap(),
            );

            database::create_edges(from, t, to);
        }

        // TODO: Insert metadata concurrently using threads
        // Store the metadata for vertices as well
        for (line_number, meta) in self.meta_list.iter() {
            let fid = self.edge_list[*line_number][0];
            let tid = self.edge_list[*line_number][0];
            let typ = &self.type_list[*line_number].1;
            let fromuid = *uuid_map.get(&fid).unwrap();
            let touid = *uuid_map.get(&tid).unwrap();

            for (name, value) in meta.into_iter() {
                database::set_edge_metadata(
                    fromuid,
                    typ.to_string(),
                    touid,
                    (name.to_owned(), value.to_owned()),
                );
            }
        }
    }
}

// Import trait implementation for node importer
impl Import for NodeImporter {
    // To be called for every column in every line
    // Nodes Step 1
    fn insert_data(&mut self, name: &str, data: ParsedColumn, _line: usize) {
        let mut meta = String::from("");
        // TODO: insert match to a function and recognize name from the third char
        match name {
            "n_id" => {
                if let ParsedColumn::Numeric(x) = data {
                    self.node_list.push(x)
                } else {
                    panic!("could not parse n_id");
                }
            }
            "n_type" => {
                if let ParsedColumn::Text(x) = data {
                    self.type_list.push((self.current_id, x))
                } else {
                    panic!("could not parse n_type");
                }
            }
            _ => {
                if let ParsedColumn::Meta(x) = data {
                    meta = x.to_owned()
                } else {
                    panic!("could not parse meta");
                }
            }
        };
        // If meta variable has data, pass it to the metadata list
        if !meta.is_empty() {
            let mut meta_map = HashMap::new();
            meta_map.insert(name.to_string(), meta);
            self.meta_list.push((self.current_id, meta_map));
        }
        self.current_id = *self.node_list.last().unwrap();
    }

    fn add_dummy_type(&mut self, _id: u32, typ: &str) {
        self.type_list.push((self.current_id, typ.to_string()));
    }

    fn get_importer_type(&self) -> Importer {
        Importer::NodeImporter
    }
}

impl NodeImporter {
    pub fn new() -> NodeImporter {
        NodeImporter {
            node_list: Vec::new(),
            type_list: Vec::new(),
            meta_list: Vec::new(),
            current_id: 0,
        }
    }

    // Nodes Step 2
    pub fn generate_id_map(&mut self, uuid_map: &mut HashMap<u32, Uuid>) -> () {
        let mut a: Vec<u32> = Vec::new();
        for id in self.node_list.iter() {
            a.push(*id);
        }
        a.sort();
        a.dedup();

        for element in a.into_iter() {
            let uuid = util::generate_uuid_v1();
            uuid_map.insert(element, uuid);
        }
    }

    // Nodes Step 3
    pub fn insert_to_db(&self, uuid_map: &HashMap<u32, Uuid>) -> () {
        println!("Storing vertices to database...");

        // PENDING: Combine two following loops into one
        for (id, t) in self.type_list.iter() {
            database::create_vertices((*uuid_map.get(id).unwrap(), t.to_owned()));
        }

        // TODO: Insert metadata concurrently using threads
        // Store the metadata for vertices as well
        for (id, meta) in self.meta_list.iter() {
            let uid = *uuid_map.get(&id).unwrap();
            for (name, value) in meta.into_iter() {
                database::set_vertex_metadata(Some(uid), (name.to_owned(), value.to_owned()));
            }
        }
    }
}
