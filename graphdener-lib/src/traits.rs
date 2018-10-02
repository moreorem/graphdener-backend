use io::filehandling::ParsedColumn;
use io::importer::Importer;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait Import {
    fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize);

    fn add_dummy_type(&mut self, external_id: u32, typ: &str);

    fn get_importer_type(&self) -> Importer;
}
