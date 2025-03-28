#[derive(Default, Debug)]
pub struct CodeStats {
    loc: usize,
    todo: usize,
    fixme: usize,
    structs: usize,
    fns: usize,
    impls: usize,
    macros: usize,
}

impl CodeStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_loc(&mut self, lines: usize) {
        self.loc += lines;
    }

    pub fn add_todo(&mut self, lines: usize) {
        self.todo += lines;
    }

    pub fn add_fixme(&mut self, lines: usize) {
        self.fixme += lines;
    }

    pub fn add_structs(&mut self, lines: usize) {
        self.structs += lines;
    }

    pub fn add_fns(&mut self, lines: usize) {
        self.fns += lines;
    }

    pub fn add_impls(&mut self, lines: usize) {
        self.impls += lines;
    }

    pub fn add_macros(&mut self, lines: usize) {
        self.macros += lines;
    }

    pub fn loc(&self) -> usize {
        self.loc
    }

    pub fn todo(&self) -> usize {
        self.todo
    }

    pub fn fixme(&self) -> usize {
        self.fixme
    }

    pub fn structs(&self) -> usize {
        self.structs
    }

    pub fn fns(&self) -> usize {
        self.fns
    }

    pub fn impls(&self) -> usize {
        self.impls
    }

    pub fn macros(&self) -> usize {
        self.macros
    }
}
