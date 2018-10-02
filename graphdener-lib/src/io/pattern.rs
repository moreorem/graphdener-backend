pub enum ImportType<'a> {
    Dual(DualPattern<'a>),
    Unified(SinglePattern<'a>),
}

pub struct DualPattern<'a> {
    pub file_path: [&'a str; 2],
    pub expression: [&'a str; 2],
    pub n_names: Vec<(&'a str, &'a str)>,
    pub e_names: Vec<(&'a str, &'a str)>,
}

pub struct SinglePattern<'a> {
    pub file_path: &'a str,
    pub expression: &'a str,
    pub col_names: Vec<(&'a str, &'a str)>,
}

impl<'a> ImportType<'a> {
    pub fn create_import(
        is_single_path: bool,
        path: Vec<&'a str>,
        patterns: Vec<&'a str>,
        names: Vec<(&'a str, &'a str)>,
    ) -> ImportType<'a> {
        // Distinguish between single or unified import
        if is_single_path {
            ImportType::Unified(SinglePattern::new(path[0], patterns[0], names))
        } else {
            ImportType::Dual(DualPattern::new(
                [path[0], path[1]],
                [patterns[0], patterns[1]],
                split_col_names(names),
            ))
        }
    }
}

impl<'a> DualPattern<'a> {
    fn new(
        file_path: [&'a str; 2],
        expression: [&'a str; 2],
        col_names: (Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>),
    ) -> DualPattern<'a> {
        let (n_names, e_names) = col_names;

        DualPattern {
            file_path,
            expression,
            n_names,
            e_names,
        }
    }
}

impl<'a> SinglePattern<'a> {
    fn new(
        file_path: &'a str,
        expression: &'a str,
        col_names: Vec<(&'a str, &'a str)>,
    ) -> SinglePattern<'a> {
        SinglePattern {
            file_path,
            expression,
            col_names,
        }
    }
}

fn split_col_names<'a>(
    names: Vec<(&'a str, &'a str)>,
) -> (Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>) {
    // Split column names to node information and edge information
    let mut n_names: Vec<(&str, &str)> = Vec::new();
    let mut e_names: Vec<(&str, &str)> = Vec::new();
    for name in names.into_iter() {
        if &name.0[0..2] == "n_" {
            n_names.push((name.0, name.1));
        } else if &name.0[0..2] == "e_" {
            e_names.push((name.0, name.1));
        } else {
            panic!("Unrecognizable column name");
        }
    }

    (n_names, e_names)
}
