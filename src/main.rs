use clap::Parser;
use env_logger::Builder;
use regex::Regex;
use std::{
    cell::Cell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

thread_local! {
    static HIDDEN: Cell<bool> = Cell::new(false);
    static DOCS: Cell<bool> = Cell::new(false);
    static COMMENTS: Cell<bool> = Cell::new(false);
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'e', long = "extension")]
    extension: Option<String>,

    #[arg(short = 'p', long = "path")]
    path: Option<PathBuf>,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short = 'a', long = "hidden")]
    hidden: bool,

    #[arg(short = 'd', long = "docs")]
    docs: bool,

    #[arg(short = 'c', long = "comments")]
    comments: bool,
}

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

fn visit_dir<'a>(
    path: &Path,
    ext: &str,
    gitignore_map: &mut HashMap<PathBuf, Vec<String>>,
) -> std::io::Result<usize> {
    let mut lines = 0usize;
    if path.is_dir() {
        log::debug!(
            "Old gitignore passed into {:?} dir: {:?}",
            path.file_name(),
            gitignore_map
        );
        let ignore_vec = get_gitignore(path);
        if ignore_vec.len() > 0 {
            gitignore_map.insert(path.to_path_buf().clone(), get_gitignore(path));
            log::debug!("Added new Gitignore in new dir: {:?}", gitignore_map);
        } else {
            log::debug!("No gitignore in dir: {:?}", path.file_name());
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if !HIDDEN.get()
                && path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or("")
                    .starts_with('.')
            {
                {
                    continue;
                }
            }

            let contains = gitignore_map.iter().any(|(_k, v)| {
                v.contains(&path.file_name().unwrap().to_str().unwrap().to_string())
            });

            if contains {
                log::info!("Ignored file: {:?}", path.file_name().unwrap());
                continue;
            }

            if path.is_dir() {
                log::info!("Dir name {:?}", path.file_name().unwrap());
                lines += visit_dir(&path, ext, gitignore_map)?;
            } else {
                if path.file_name().unwrap().to_str().unwrap().ends_with(ext) {
                    log::debug!("Good file with good ext");
                    log::debug!("Filename name {:?}", path.file_name().unwrap());
                    lines += count_lines(&path);
                } else {
                    continue;
                }
            }
            log::info!("Total amount of lines: {}\n", &lines);
        }
        gitignore_map.remove(&path.to_path_buf());
    } else {
        // Can get here only if user provide path which is not directory
        if path.file_name().unwrap().to_str().unwrap().ends_with(ext) {
            log::debug!("Good file with good ext");
            log::debug!("Filename name {:?}", path.file_name().unwrap());
            lines += count_lines(&path);
        } else {
            panic!();
        }
    }
    log::info!("Getting out of {:?}", path.file_name());
    log::info!("Total lines in {:?}: {}\n", path.file_name(), lines);

    Ok(lines)
}

fn count_lines(file: &Path) -> usize {
    let file_str = fs::read_to_string(file).unwrap();

    let mut lines = file_str.lines().collect::<Vec<&str>>();

    let mut i = 0;
    let mut in_multi_comment = false;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            lines.remove(i);
            continue;
        }

        if !COMMENTS.get() {
            if line.len() < 2 && !in_multi_comment {
                i += 1;
                continue;
            }

            if in_multi_comment {
                if line.len() >= 2 && line[line.len() - 2..=line.len() - 1] == *"*/" {
                    in_multi_comment = false;
                }

                lines.remove(i);

                continue;
            }

            if line[0..=1] == *"/*" {
                lines.remove(i);
                in_multi_comment = true;
                continue;
            }

            if line[0..=1] == *"//" && line.chars().nth(2) != Some('/') {
                lines.remove(i);
                continue;
            }
        }

        if !DOCS.get() {
            if line.len() >= 3 && (line[0..=2] == *"///" || line[0..=2] == *"//!") {
                lines.remove(i);
                continue;
            }
        }

        i += 1;
    }

    // println!("{}", lines.join("\n"));

    log::info!("Lines in {:?}: {}", file.file_name(), lines.len());
    lines.len()
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        Builder::new()
            .filter(None, log::LevelFilter::Info)
            // .format(|buf, record| {
            //     let info_style = buf.default_level_style(log::Level::Info);
            //     writeln!(
            //         buf,
            //         "INFO: {info_style:#}{}{info_style}",
            //         record.args()
            //     )
            // })
            .init();
    } else {
        Builder::new().filter(None, log::LevelFilter::Off).init();
    }

    let path = match cli.path {
        Some(path) => path,
        None => std::env::current_dir().expect("Invalid directory"),
    };

    let ext = match cli.extension {
        Some(ext) => ext,
        None => ".rs".to_string(),
    };

    HIDDEN.set(cli.hidden);
    DOCS.set(cli.docs);
    COMMENTS.set(cli.comments);

    log::info!("Path: {}", path.to_str().unwrap());
    log::info!("File extension: {}", ext);
    let mut gitignore_map: HashMap<PathBuf, Vec<String>> = HashMap::new();
    let res = visit_dir(&path, &ext, &mut gitignore_map);

    match res {
        Ok(lines) => println!("{}", lines),
        Err(e) => println!("{}", e),
    }
}
