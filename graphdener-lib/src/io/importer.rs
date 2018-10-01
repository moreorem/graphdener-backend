use super::super::commands::database;
use super::filehandling::ParsedColumn;
use graphdenerdb::util;
use graphdenerdb::{Datastore, EdgeKey, EdgeQuery, Vertex, VertexQuery};
use statics;
use std::collections::{BTreeMap, HashMap};
use traits::Import;
use uuid::Uuid;

pub enum Importer {
    NodeImporter,
    EdgeImporter,
    UnifiedImporter,
}

pub struct UnifiedImporter {
    node_list: Vec<u32>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
    current_id: u32,
}

pub struct NodeImporter {
    has_type: bool,
    node_list: Vec<u32>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
    current_id: u32,
}

// Because there is no edge id usually we create it using line number
pub struct EdgeImporter {
    has_type: bool,
    edge_id: Vec<u32>,
    edge_list: Vec<[u32; 2]>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
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
                    self.type_list.push((line as u32, x))
                } else {
                    panic!("unknown type type");
                }
            }
            _ => {
                if let ParsedColumn::Meta(x) = data {
                    meta = x.to_owned()
                } else {
                    panic!("unknown meta type");
                }
            }
        };
        // If meta variable has data, pass it to the metadata list
        if !meta.is_empty() {
            let mut meta_map = HashMap::new();
            meta_map.insert(name.to_string(), meta);
            self.meta_list.push((line as u32, meta_map));
        }
        // If temporary from-to line is filled empty the contents into the edgelist
        if let [Some(x), Some(y)] = self.temp_fromto {
            self.edge_list.push([x, y]);
            self.temp_fromto = [None, None];
        }
        // TESTME: debugging
        // let msg = database::get_edge_metadata(None, "e_label".to_string(), None);
        let msg = database::get_graph_edges(None);
        println!("{:?}", msg);
    }
}

impl EdgeImporter {
    pub fn new(has_type: bool) -> EdgeImporter {
        EdgeImporter {
            has_type,
            edge_id: Vec::new(),
            edge_list: Vec::new(),
            type_list: Vec::new(),
            meta_list: Vec::new(),
            temp_fromto: [None, None],
        }
    }

    // Sends the edge data to database
    pub fn insert_to_db(&self, uuid_map: &HashMap<u32, Uuid>) -> () {
        println!("Storing edges to database...");

        // PENDING: fill type_list with something in case there are no values
        let data: Vec<(Uuid, String, Uuid)> = self
            .type_list
            .iter()
            .map(|(line_number, typ)| {
                (
                    *uuid_map
                        .get(&self.edge_list[*line_number as usize][0]) // FIXME: Out of bounds
                        .unwrap(),
                    typ.to_owned(),
                    *uuid_map
                        .get(&self.edge_list[*line_number as usize][1])
                        .unwrap(),
                )
            })
            .collect();

        for triplet in data.into_iter() {
            let (from, t, to) = triplet;
            database::create_edges(from, t, to);
        }

        // let data: Vec<(Uuid, t, Uuid, String, String)> = self.meta_list.iter().map(|(line_number, meta)| )
        // Store the metadata for vertices as well
        for (line_number, meta) in self.meta_list.iter() {
            let fid = self.edge_list[*line_number as usize][0];
            let tid = self.edge_list[*line_number as usize][0];
            let typ = &self.type_list[*line_number as usize].1;
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

        // TESTME: debugging
        // let msg = database::get_edge_metadata(None, "e_label".to_string(), None);
        let msg = database::get_graph_edges(None);
        println!("{:?}", msg);
    }
}

// Import trait implementation for node importer
impl Import for NodeImporter {
    // To be called for every column in every line
    // Nodes Step 1
    fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize) {
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
}

impl NodeImporter {
    pub fn new(has_type: bool) -> NodeImporter {
        NodeImporter {
            has_type,
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

        let data: Vec<(Uuid, String)> = self
            .type_list
            .iter()
            .map(|(x, y)| (*uuid_map.get(&x).unwrap(), y.to_owned()))
            .collect();

        for pair in data.into_iter() {
            database::create_vertices(pair);
        }

        // Store the metadata for vertices as well
        for (id, meta) in self.meta_list.iter() {
            let uid = *uuid_map.get(&id).unwrap();
            for (name, value) in meta.into_iter() {
                database::set_vertex_metadata(Some(uid), (name.to_owned(), value.to_owned()));
            }
        }

        // TESTME: debugging
        // let msg = database::get_vertex_metadata(None, "n_label");
        let msg = database::get_graph_vertices(None);
        println!("{:?}", msg);
    }
}

// impl Import for UnifiedImporter {
//     fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize) {
//         let mut n_meta = String::from("");
//         let mut e_meta = String::from("");

//         // TODO: insert match to a function and recognize name from the third char
//         if name[..2] == *"n_" {
//             match name {
//                 "n_id" => {
//                     if let ParsedColumn::Numeric(x) = data {
//                         self.node_list.push(x)
//                     } else {
//                         panic!("could not parse n_id");
//                     }
//                 }
//                 "n_type" => {
//                     if let ParsedColumn::Text(x) = data {
//                         self.type_list.push((self.current_id, x))
//                     } else {
//                         panic!("could not parse n_type");
//                     }
//                 }
//                 _ => {
//                     if let ParsedColumn::Meta(x) = data {
//                         n_meta = x.to_owned()
//                     } else {
//                         panic!("could not parse node meta");
//                     }
//                 }
//             }
//         }
//         else if name[..2] == *"e_" {
//             match name {
//                 "e_id" => {
//                 if let ParsedColumn::Numeric(x) = data {
//                     // Dummy value, we probably wont use e_id because we generate our own
//                     // self.line_number.push(x)
//                 } else {
//                     panic!("unknown id type");
//                 }
//             }
//             "e_from" => {
//                 if let ParsedColumn::Numeric(x) = data {
//                     self.temp_fromto[0] = Some(x);
//                 } else {
//                     panic!("unknown source type");
//                 }
//             }
//             "e_to" => {
//                 if let ParsedColumn::Numeric(x) = data {
//                     self.temp_fromto[1] = Some(x);
//                 } else {
//                     panic!("unknown target type");
//                 }
//             }
//             "e_type" => {
//                 if let ParsedColumn::Text(x) = data {
//                     self.type_list.push((line as u32, x))
//                 } else {
//                     panic!("unknown type type");
//                 }
//             }
//             _ => {
//                 if let ParsedColumn::Meta(x) = data {
//                     e_meta = x.to_owned()
//                 } else {
//                     panic!("unknown meta type");
//                 }
//             }
//           }
//         };
//         // If meta variable has data, pass it to the metadata list
//         if !n_meta.is_empty() {
//             let mut meta_map = HashMap::new();
//             meta_map.insert(name.to_string(), n_meta);
//             self.meta_list.push((self.current_id, meta_map));
//         }
//         self.current_id = *self.node_list.last().unwrap();
//     }
// }

// impl UnifiedImporter {
//     pub fn new() -> UnifiedImporter {
//         UnifiedImporter {
//             node_list: Vec<u32>,
//             type_list: Vec<(u32, String)>,
//             meta_list: Vec<(u32, HashMap<String, String>)>,
//             current_id: u32,
//         }
//     }
// }
