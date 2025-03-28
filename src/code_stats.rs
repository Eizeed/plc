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

