use crate::params::Params;

#[derive(Default, Debug)]
pub struct CodeStats {
    loc: usize,
    todo: usize,
    fixme: usize,
    structs: usize,
    fns: usize,
    impls: usize,
    macros: usize,
    comments: usize,
    docs: usize,
    empty_lines: usize,
}

macro_rules! getter_setter {
    ($field_name:ident, $setter:ident) => {
        pub fn $field_name(&self) -> usize {
            self.$field_name
        }
        pub fn $setter(&mut self) {
            self.$field_name += 1;
        }
    };
}

impl CodeStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print(&self, params: &Params) {
        let loc = self.loc();
        let todo = self.todo();
        let fixme = self.fixme();

        let structs = self.structs();
        let functions = self.fns();
        let impl_blocks = self.impls();
        let macros = self.macros();

        let empty_lines = self.empty_lines();
        let comments = self.comments();
        let docs = self.docs();

        if params.json {
            let mut res = String::from("{");
            res.push_str(&format!(r#""loc": {}"#, loc));

            if params.todo {
                res.push_str(&format!(r#","todo": {}"#, todo));
            }
            if params.fixme {
                res.push_str(&format!(r#","fixme": {}"#, fixme));
            }

            if params.units {
                res.push_str(&format!(r#","structs": {}"#, structs));
                res.push_str(&format!(r#","functions": {}"#, functions));
                res.push_str(&format!(r#","impl_blocks": {}"#, impl_blocks));
                res.push_str(&format!(r#","macros": {}"#, macros));
            }

            if params.ratio {
                let unit = (empty_lines + comments + docs + loc) as f64 / 100.0;
                let empty_lines_ratio = empty_lines as f64 / unit;
                let comments_ratio = comments as f64 / unit;
                let docs_ratio = docs as f64 / unit;
                let loc_ratio = loc as f64 / unit;

                res.push_str(&format!(
                    r#","empty_lines_ratio": "{:.3}%""#,
                    empty_lines_ratio
                ));
                res.push_str(&format!(r#","comments_ratio": "{:.3}%""#, comments_ratio));
                res.push_str(&format!(r#","docs_ratio": "{:.3}%""#, docs_ratio));
                res.push_str(&format!(r#","loc_ratio": "{:.3}%""#, loc_ratio));
            }

            res.push('}');
            println!("{}", res);
        } else {
            println!("{}", loc);

            if params.todo {
                println!("todo: {}", todo);
            }
            if params.fixme {
                println!("fixme: {}", fixme);
            }

            if params.units {
                println!("structs: {}", structs);
                println!("functions: {}", functions);
                println!("impl blocks: {}", impl_blocks);
                println!("macros: {}", macros);
            }

            if params.ratio {
                let unit = (empty_lines + comments + docs + loc) as f64 / 100.0;
                let empty_lines_ratio = empty_lines as f64 / unit;
                let comments_ratio = comments as f64 / unit;
                let docs_ratio = docs as f64 / unit;
                let loc_ratio = loc as f64 / unit;

                println!("empty lines: {}", empty_lines_ratio);
                println!("comments: {}", comments_ratio);
                println!("docs: {}", docs_ratio);
                println!("loc: {}", loc_ratio);
            }
        }
    }

    pub fn add_loc(&mut self, lines: usize) {
        self.loc += lines;
    }

    pub fn loc(&self) -> usize {
        self.loc
    }

    getter_setter!(todo, add_todo);
    getter_setter!(fixme, add_fixme);
    getter_setter!(structs, add_structs);
    getter_setter!(fns, add_fns);
    getter_setter!(impls, add_impls);
    getter_setter!(macros, add_macros);
    getter_setter!(comments, add_comments);
    getter_setter!(docs, add_docs);
    getter_setter!(empty_lines, add_empty_lines);
}
