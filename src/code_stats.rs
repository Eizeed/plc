#[derive(Default, Debug)]
pub struct CodeStats {
    loc: usize,
    todo: usize,
    fixme: usize,
}

impl CodeStats {
    pub fn new() -> Self {
        Self {
            loc: 0,
            todo: 0,
            fixme: 0,
        }
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

    pub fn loc(&self) -> usize {
        self.loc
    }

    pub fn todo(&self) -> usize {
        self.todo
    }

    pub fn fixme(&self) -> usize {
        self.fixme
    }
}
