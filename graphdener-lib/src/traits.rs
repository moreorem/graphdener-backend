use io::filehandling::ParsedColumn;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub trait Import {
    fn insert_data(&mut self, name: &str, data: ParsedColumn, line: usize);
}
