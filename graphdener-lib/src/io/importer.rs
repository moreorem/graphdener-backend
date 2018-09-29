use super::super::commands::database;
use graphdenerdb::util;
use graphdenerdb::{
    Datastore, EdgeKey, EdgeQuery, Transaction, Type, Vertex, VertexQuery,
};
use statics;
use std::collections::{HashMap, BTreeMap};
use uuid::Uuid;
use super::filehandling::ParsedColumn;
// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

// TODO: Simplify importers. Only write to database and use graphcontainer afterwards to create Uuid_map and edgelists
pub enum Importer {
    NodeImporter,
    EdgeImporter,
    UnifiedImporter,
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

    pub fn update(&mut self, conn: (u32, u32, u32, &str, &str, u8)) {
        let a = (
            conn.0,
            conn.1,
            conn.2,
            conn.3.to_string(),
            conn.4.to_string(),
            conn.5,
        );
        self.unified_list.push(a);
    }
}

pub struct EdgeImporter {
    pub edge_list: Vec<(u32, u32, u32, String, String, u8)>,
    uuid_map: HashMap<u32, Uuid>,
}

impl EdgeImporter {
    pub fn new() -> EdgeImporter {
        EdgeImporter {
            edge_list: Vec::new(),
            uuid_map: HashMap::new(),
        }
    }

    pub fn update(&mut self, conn: (u32, u32, u32, &str, &str, u8)) {
        let a = (
            conn.0,
            conn.1,
            conn.2,
            conn.3.to_string(),
            conn.4.to_string(),
            conn.5,
        );
        self.edge_list.push(a);
    }

    // To be called for every column in every line
    // pub fn insert_data(&mut self, name: &str, data: ParsedColumn)
    // {
    //     let mut meta = String::from("");
    //     match name {
    //         "e_from" => { if let ParsedColumn::Numeric(x) = data {self.node_list.push(x)} else {panic!("unknown column type");}},
    //         "e_to" => { if let ParsedColumn::Numeric(x) = data {self.node_list.push(x)} else {panic!("unknown column type");}},
    //         "e_type" => { if let ParsedColumn::Text(x) = data {self.type_list.push(x)} else {panic!("unknown column type");}},
    //         _ => { if let ParsedColumn::Meta(x) = data { meta = x} else {panic!("unknown column type");}}
    //     };

    //     // If meta variable has data, pass it to the metadata list
    //     if !meta.is_empty(){
    //         let a = self.meta_list.entry(name.to_string()).or_insert(vec![meta.clone()]);
    //         a.push(meta);
    //     }
    // }

    // Use this method only when there is only an edge list file
    pub fn create_edges(&self, uuid_map: &HashMap<u32, Uuid>) -> () {
        println!("Storing edges to database...");

        // iterate over every uuid in the hashmap and create each unique node into the db
        for val in self.edge_list.iter() {
            // uuid_list.push(val);
            let target = uuid_map.get(&val.2).unwrap();
            let source = uuid_map.get(&val.1).unwrap();
            let t = Type::new(val.4.to_owned()).unwrap();
            let l = &val.3;
            let w = &val.5;

            database::create_edges(
                *target,
                t,
                *source,
                Some(l.to_string()),
                Some(w.to_string()),
            );
        }
    }
}

pub struct NodeImporter {
    node_list: Vec<u32>,
    pub type_list: Vec<(u32, String)>,
    pub meta_list: BTreeMap<u32, HashMap<String, String>>,
    uuid_map: HashMap<u32, Uuid>, // PENDING: Deprecated delete if sure
    current_id: u32
}

impl NodeImporter {
    pub fn new() -> NodeImporter {
        NodeImporter {
            node_list: Vec::new(),
            type_list: Vec::new(),
            //type_map: HashMap::new(),
            uuid_map: HashMap::new(),
            meta_list: BTreeMap::new(),
            current_id: 0
        }
    }

    // To be called for every column in every line
    // Nodes Step 1
    pub fn insert_data(&mut self, name: &str, data: ParsedColumn)
    {
        let mut meta = String::from("");
        match name {
            "n_id" => { 
                if let ParsedColumn::Numeric(x) = data {self.node_list.push(x)} 
                else {panic!("unknown column type");}
            },
            "n_type" => { 
                if let ParsedColumn::Text(x) = data {self.type_list.push((self.current_id, x))} 
                else {panic!("unknown column type");} 
            },
            _ => { if let ParsedColumn::Meta(x) = data { meta = x.to_owned()} else {panic!("unknown column type");}}
        };

        // If meta variable has data, pass it to the metadata list
        if !meta.is_empty(){
            let mut meta_map = HashMap::new();
            meta_map.insert(name.to_string(), meta);
            self.meta_list.insert(self.current_id, meta_map);
        }
        self.current_id = *self.node_list.last().unwrap();
    }

    // Nodes Step 2
    pub fn generate_id_map(&mut self) -> Result<HashMap<u32, Uuid>, bool> {
        let mut a: Vec<u32> = Vec::new();
        let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();

        for id in self.node_list.iter() {
            a.push(*id);
        }

        // probably would be faster if map function is used
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
    pub fn create_vertices(&self) -> () {
        println!("Storing vertices to database...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut v: Vertex;

        let data: Vec<(Uuid, String)> = self.type_list.iter().map(|(x,y)| (*self.uuid_map.get(&x).unwrap(), y.to_owned())).collect();

        for pair in data.iter() {
            v = Vertex::with_id(pair.0, Type::new(pair.1.to_string()).unwrap());
            trans.create_vertex(&v);
        }

        // Store the metadata for vertices as well
        for (id, meta) in self.meta_list.iter() {
            let uid = *self.uuid_map.get(&id).unwrap();
            for (name, value) in meta.into_iter() {
                database::set_vertex_metadata(Some(uid), (name.to_owned(), value.to_owned()));
            }
        }
        let msg = trans.get_vertex_count();
    }
}

// Initializes the vertex attributes to be changed in every frame
pub fn initialize_spatial() {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v = VertexQuery::All {
        start_id: None,
        limit: 1000000000,
    };
    trans.set_vertex_metadata(&v, "pos", &json!([0., 0.]));
    trans.set_vertex_metadata(&v, "label", &json!(""));
}
