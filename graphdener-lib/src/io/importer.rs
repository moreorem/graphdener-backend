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

// Because there is no edge id usually we create it using line number
pub struct EdgeImporter<'a> {
    uuid_map: &'a HashMap<u32, Uuid>,
    edge_id: Vec<u32>,
    edge_list: Vec<[u32; 2]>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
    temp_fromto: [Option<u32>; 2],
}

impl<'a> EdgeImporter<'a> {
    pub fn new(uuid_map: &HashMap<u32, Uuid>) -> EdgeImporter {
        EdgeImporter {
            uuid_map,
            edge_id: Vec::new(),
            edge_list: Vec::new(),
            type_list: Vec::new(),
            meta_list: Vec::new(),
            temp_fromto: [None, None],
        }
    }

    // Sends the edge data to database
    pub fn insert_to_db(&self) -> () {
        println!("Storing edges to database...");

        let data: Vec<(Uuid, String, Uuid)> = self
            .type_list
            .iter()
            .map(|(line_number, typ)| {
                (
                    *self
                        .uuid_map
                        .get(&self.edge_list[*line_number as usize][0]) // FIXME: Out of bounds
                        .unwrap(),
                    typ.to_owned(),
                    *self
                        .uuid_map
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
            let fromuid = *self.uuid_map.get(&fid).unwrap();
            let touid = *self.uuid_map.get(&tid).unwrap();

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
        let msg = database::get_edge_metadata(None, "e_label".to_string(), None);
        let msg = database::get_graph_edges(None);
        println!("{:?}", msg);
    }
}

// Use this method only when there is only an edge list file
// pub fn create_edges(&self, uuid_map: &HashMap<u32, Uuid>) -> () {
//     println!("Storing edges to database...");

//     // iterate over every uuid in the hashmap and create each unique node into the db
//     for val in self.edge_list.iter() {
//         // uuid_list.push(val);
//         let target = uuid_map.get(&val.2).unwrap();
//         let source = uuid_map.get(&val.1).unwrap();
//         let t = Type::new(val.4.to_owned()).unwrap();
//         let l = &val.3;
//         let w = &val.5;

//         database::create_edges(
//             *target,
//             t,
//             *source,
//             Some(l.to_string()),
//             Some(w.to_string()),
//         );
//     }
// }

// Import trait implementation for edge importer
impl<'a> Import for EdgeImporter<'a> {
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
                println!("{:?}", &data);
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
        // self.line_number = *self.line_number.last().unwrap();
        println!("{}", line);
    }
}

pub struct NodeImporter {
    node_list: Vec<u32>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: Vec<(u32, HashMap<String, String>)>,
    uuid_map: HashMap<u32, Uuid>,
    current_id: u32,
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
                    panic!("unknown column type");
                }
            }
            "n_type" => {
                if let ParsedColumn::Text(x) = data {
                    self.type_list.push((self.current_id, x))
                } else {
                    panic!("unknown column type");
                }
            }
            _ => {
                if let ParsedColumn::Meta(x) = data {
                    meta = x.to_owned()
                } else {
                    panic!("unknown column type");
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
    pub fn new() -> NodeImporter {
        NodeImporter {
            node_list: Vec::new(),
            type_list: Vec::new(),
            uuid_map: HashMap::new(),
            meta_list: Vec::new(),
            current_id: 0,
        }
    }

    // Nodes Step 2
    pub fn generate_id_map(&mut self) -> Result<HashMap<u32, Uuid>, bool> {
        let mut a: Vec<u32> = Vec::new();
        let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();

        for id in self.node_list.iter() {
            a.push(*id);
        }

        a.sort();
        a.dedup();

        for element in a.into_iter() {
            let uuid = util::generate_uuid_v1();
            uuid_map.insert(element, uuid);
        }
        // Update instance map
        self.uuid_map = uuid_map.clone();
        Ok(uuid_map)
    }

    // Nodes Step 3
    pub fn insert_to_db(&self) -> () {
        println!("Storing vertices to database...");

        let data: Vec<(Uuid, String)> = self
            .type_list
            .iter()
            .map(|(x, y)| (*self.uuid_map.get(&x).unwrap(), y.to_owned()))
            .collect();

        for pair in data.into_iter() {
            database::create_vertices(pair);
        }

        // Store the metadata for vertices as well
        for (id, meta) in self.meta_list.iter() {
            let uid = *self.uuid_map.get(&id).unwrap();
            for (name, value) in meta.into_iter() {
                database::set_vertex_metadata(Some(uid), (name.to_owned(), value.to_owned()));
            }
        }

        // TESTME: debugging
        // let msg = database::get_vertex_metadata(None, "n_label");
        // let msg = database::get_graph_vertices(None);
        // println!("{:?}", msg);
    }
}

pub struct UnifiedImporter {
    pub unified_list: Vec<(u32, u32, u32, String, String, u8)>,
    uuid_map: HashMap<u32, Uuid>,
}

impl UnifiedImporter {
    pub fn new() -> UnifiedImporter {
        UnifiedImporter {
            unified_list: Vec::new(),
            uuid_map: HashMap::new(),
        }
    }

    // pub fn update(&mut self, conn: (u32, u32, u32, &str, &str, u8)) {
    //     let a = (
    //         conn.0,
    //         conn.1,
    //         conn.2,
    //         conn.3.to_string(),
    //         conn.4.to_string(),
    //         conn.5,
    //     );
    //     self.unified_list.push(a);
    // }
}
