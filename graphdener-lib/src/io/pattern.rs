pub struct DualPattern<'a> {
    pub file_path: [&'a str; 2],
    pub expression: [&'a str; 2],
}

pub struct SinglePattern<'a> {
    pub file_path: &'a str,
    pub expression: &'a str,
}

pub enum ImportType<'a> {
    Dual(DualPattern<'a>),
    Unified(SinglePattern<'a>),
}

impl<'a> ImportType<'a> {
    pub fn create_import(
        is_single_path: bool,
        path: Vec<&'a str>,
        patterns: Vec<&'a str>,
    ) -> ImportType<'a> {
        if is_single_path {
            ImportType::Unified(SinglePattern::new(path[0], patterns[0]))
        } else {
            ImportType::Dual(DualPattern::new(
                [path[0], path[1]],
                [patterns[0], patterns[1]],
            ))
        }
    }
}

impl<'a> DualPattern<'a> {
    fn new(file_path: [&'a str; 2], expression: [&'a str; 2]) -> DualPattern<'a> {
        DualPattern {
            file_path,
            expression,
        }
    }
}

impl<'a> SinglePattern<'a> {
    fn new(file_path: &'a str, expression: &'a str) -> SinglePattern<'a> {
        SinglePattern {
            file_path,
            expression,
        }
    }
}
