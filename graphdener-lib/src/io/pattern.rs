pub struct InitPattern<'a> {
    pub file_path: [&'a str; 2],
    pub expression: [&'a str; 2],
}

impl<'a> InitPattern<'a> {
    pub fn create_import(
        path: Vec<&'a str>,
        patterns: Vec<&'a str>,
        // names: Vec<(&'a str, &'a str)>,
    ) -> InitPattern<'a> {
        // Distinguish between single or unified import
        InitPattern::new(
            [path[0], path[1]],
            [patterns[0], patterns[1]],
            // split_col_names(names),
        )
    }

    fn new(
        file_path: [&'a str; 2],
        expression: [&'a str; 2],
        // col_names: (Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>),
    ) -> InitPattern<'a> {
        // let (n_names, e_names) = col_names;

        InitPattern {
            file_path,
            expression,
            // n_names,
            // e_names,
        }
    }
}
