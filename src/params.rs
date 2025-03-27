use std::{
    env,
    path::{Path, PathBuf},
};

use crate::args::Args;

pub struct Params {
    extensions: Vec<String>,
    path: PathBuf,
    verbose: bool,
    hidden: bool,
    docs: bool,
    comments: bool,
    fixme: bool,
    todo: bool,
}

impl Params {
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn docs(&self) -> bool {
        self.docs
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn fixme(&self) -> bool {
        self.fixme
    }

    pub fn todo(&self) -> bool {
        self.todo
    }
}

impl<'a> From<Args> for Params {
    fn from(value: Args) -> Self {
        let extensions: Vec<String> = match value.extensions {
            Some(extensions) => extensions,
            None => vec![".rs".to_string()],
        };

        let path = match value.path {
            Some(p) => p,
            None => env::current_dir().expect("Provided path is invalid"),
        };

        Self {
            extensions,
            path,
            verbose: value.verbose,
            hidden: value.hidden,
            docs: value.hidden,
            comments: value.comments,
            fixme: value.fixme,
            todo: value.todo,
        }
    }
}
