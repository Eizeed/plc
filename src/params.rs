use std::{env, path::PathBuf};

use crate::args::Args;

pub struct Params {
    pub extensions: Vec<String>,
    pub path: PathBuf,
    pub verbose: bool,
    pub hidden: bool,
    pub docs: bool,
    pub comments: bool,
    pub fixme: bool,
    pub todo: bool,
    pub units: bool,
    pub ratio: bool,
    pub json: bool,
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

        let units = extensions.iter().find(|str| *str == ".rs").is_some() && value.units;

        Self {
            extensions,
            path,
            verbose: value.verbose,
            hidden: value.hidden,
            docs: value.docs,
            comments: value.comments,
            fixme: value.fixme,
            todo: value.todo,
            units,
            ratio: value.ratio,
            json: value.json,
        }
    }
}
