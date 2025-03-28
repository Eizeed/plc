use args::Args;
use clap::Parser;
use count_lines::count_lines;
use env_logger::Builder;
use params::Params;
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    time::Instant,
};

mod code_stats;
use code_stats::CodeStats;

mod args;
mod count_lines;
mod params;

fn get_gitignore(dir: &Path) -> Vec<String> {
    let mut gitignore: Vec<String> = vec![];
    if dir.is_dir() {
        'outer: for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.file_name().unwrap_or_default().to_str().unwrap_or("") == ".gitignore" {
                let start_regex = Regex::new(r#"^[/\- \t]"#).unwrap();
                let end_regex = Regex::new(r#"[/ \t]+$"#).unwrap();
                let lines = fs::read_to_string(&path).unwrap();
                let lines = lines.lines();
                for line in lines {
                    if line.starts_with("#") {
                        continue;
                    }
                    let line = start_regex.replace_all(&line, "");
                    let line = end_regex.replace_all(&line, "");
                    gitignore.push(line.to_string())
                }
                break 'outer;
            }
        }
    }

    return gitignore;
}

fn visit_dir(
    params: &Params,
    path: &Path,
    stats: &mut CodeStats,
    gitignore_map: &mut HashMap<PathBuf, Vec<String>>,
) -> std::io::Result<()> {
    if path.is_dir() {
        log::debug!(
            "Old gitignore passed into {:?} dir: {:?}",
            path,
            gitignore_map
        );

        let ignore_vec = get_gitignore(path);
        if ignore_vec.len() > 0 {
            gitignore_map.insert(path.to_path_buf().clone(), ignore_vec);
            log::debug!("Added new Gitignore in new dir: {:?}", gitignore_map);
        } else {
            log::debug!("No gitignore in dir: {:?}", path.file_name());
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path: &Path = &entry.path();

            if !params.hidden
                && path
                    .file_name()
                    .ok_or(io::Error::from(ErrorKind::InvalidData))?
                    .to_str()
                    .ok_or(io::Error::from(ErrorKind::InvalidData))?
                    .starts_with('.')
            {
                continue;
            }

            let should_ignore = gitignore_map.iter().any(|(_k, v)| {
                v.contains(
                    &entry_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            });
            if should_ignore {
                log::info!("Ignored file: {:?}", entry_path.file_name().unwrap());
                continue;
            }

            if entry_path.is_dir() {
                log::info!("Dir name {:?}", entry_path.file_name().unwrap());
                visit_dir(params, &entry_path, stats, gitignore_map)?;
            } else {
                let file_name = match entry_path.file_name() {
                    Some(file_name) => file_name.to_str().unwrap(),
                    None => continue,
                };

                let matching_ext = params.extensions.iter().any(|ext| file_name.ends_with(ext));
                if matching_ext {
                    log::debug!("Good file with good ext");
                    log::debug!("Filename name {:?}", entry_path.file_name().unwrap());
                    count_lines(&entry_path, params, stats);
                } else {
                    continue;
                }
            }
            log::info!("Total amount of lines: {}\n", &stats.loc());
        }
        gitignore_map.remove(&path.to_path_buf());
    } else {
        // Can get here only if user provide path which is not directory
        log::debug!("Filename name {:?}", path.file_name().unwrap());
        count_lines(path, params, stats);
    }

    log::info!("Getting out of {:?}", path.file_name());
    log::info!("Total lines in {:?}: {}\n", path.file_name(), stats.loc());

    Ok(())
}

fn main() {
    let args = Args::parse();

    let params = Params::from(args);
    let mut code_stats = CodeStats::new();

    log::info!("Path: {}", params.path.to_str().unwrap());
    log::info!("File extensions: {}", params.extensions.join(" "));

    if params.verbose {
        Builder::new().filter(None, log::LevelFilter::Info).init();
    } else {
        Builder::new().filter(None, log::LevelFilter::Off).init();
    }

    let mut gitignore_map: HashMap<PathBuf, Vec<String>> = HashMap::new();
    let _start = Instant::now();
    let res = visit_dir(&params, &params.path, &mut code_stats, &mut gitignore_map);

    match res {
        Ok(_) => {
            println!("{}", code_stats.loc());
            if params.todo {
                println!("todos: {}", code_stats.todo())
            }
            if params.fixme {
                println!("fixmes: {}", code_stats.fixme())
            }
            if params.units {
                println!("structs: {}", code_stats.structs());
                println!("functions: {}", code_stats.fns());
                println!("impl blocks: {}", code_stats.impls());
                println!("macros: {}", code_stats.macros());
            }
            if params.ratio {
                let empty_lines = code_stats.empty_lines();
                let comments = code_stats.comments();
                let docs = code_stats.docs();
                let loc = code_stats.loc();

                let unit = (empty_lines + comments + docs + loc) as f64 / 100.0;
                println!("units: {unit}");
                let empty_lines_ratio = empty_lines as f64 / unit;
                let comments_ratio = comments as f64 / unit;
                let docs_ratio = docs as f64 / unit;
                let loc_ratio = loc as f64 / unit;

                println!("empty lines: {:.3}%", empty_lines_ratio);
                println!("comments: {:.3}%", comments_ratio);
                println!("docs: {:.3}%", docs_ratio);
                println!("lines of code: {:.3}%", loc_ratio);
            }
        }
        Err(e) => println!("{}", e),
    }
}
