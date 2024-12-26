use std::{io::Write, collections::HashMap, fs, path::{Path, PathBuf}};

use clap::Parser;
use env_logger::Builder;
use regex::Regex;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    path: PathBuf,
    extension: String,

    #[arg(short, long)]
    verbose: bool,
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

fn visit_dir<'a>(dir: &Path, ext: &str, gitignore_map: &mut HashMap<PathBuf, Vec<String>>) -> std::io::Result<usize> {
    let mut lines = 0usize;
    if dir.is_dir() {
        log::debug!("Old gitignore passed into {:?} dir: {:?}", dir.file_name(), gitignore_map);
        let ignore_vec = get_gitignore(dir);
        if ignore_vec.len() > 0 {
            gitignore_map.insert(dir.to_path_buf().clone(),get_gitignore(dir));
            log::debug!("Added new Gitignore in new dir: {:?}", gitignore_map);
        } else {
            log::debug!("No gitignore in dir: {:?}", dir.file_name());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with('.') {
                continue;
            }

            
            let contains = gitignore_map.iter().any(|(_k, v)| v.contains(&path.file_name().unwrap().to_str().unwrap().to_string()));

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
        gitignore_map.remove(&dir.to_path_buf());
    }
    log::info!("Getting out of {:?}", dir.file_name());
    log::info!("Total lines in {:?}: {}\n",dir.file_name(), lines);
    Ok(lines)
}

fn count_lines(file: &Path) -> usize {
    let file_str = fs::read_to_string(file).unwrap();

    let new_lines_re = Regex::new(r#"\n{2,}"#).unwrap();
    let multi_comment = Regex::new(r#"/\*[\s\S]*\*/\s"#).unwrap();
    let single_comment = Regex::new(r#"\s*//.*"#).unwrap();

    let file_str = new_lines_re.replace_all(&file_str, "\n");
    let file_str = multi_comment.replace_all(&file_str, "");
    let mut lines: Vec<&str> = file_str.lines().collect();

    let mut i = 0;
    let mut len = lines.len();
    while i < len {
        if single_comment.is_match(lines[i]) {
            lines.remove(i);
            len -= 1;
            continue;
        }
        i += 1;
    }
    log::info!("Lines in {:?}: {}",file.file_name(), lines.clone().len());
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
        Builder::new()
            .filter(None, log::LevelFilter::Off)
            .init();
    }

    match fs::metadata(&cli.path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                log::info!("Path: {}", cli.path.to_str().unwrap());
                log::info!("File extension: {}", cli.extension);
                let mut gitignore_map: HashMap<PathBuf, Vec<String>>= HashMap::new();
                let lines = visit_dir(&cli.path, &cli.extension, &mut gitignore_map);

                // Force to use white color in terminal
                // because logger makes it green
                println!("\x1b[0m{}\x1b[0m", lines.unwrap());
            }
        },
        Err(_) => ()
    }
}
