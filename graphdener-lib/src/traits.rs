use io::filehandling::ParsedColumn;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait Import {
    fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize);

    fn add_dummy_type(&self, typ: &str, type_list: Vec<(u32, String)>) {
        self.type_list.push(self.id)
    }
}
