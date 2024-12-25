use std::{fs, path::{Path, PathBuf}};

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
                let lines = fs::read_to_string(&path).unwrap();
                let lines = lines.lines();
                for line in lines {
                    gitignore.push(line.to_string())
                }
                break 'outer;
            }
        }
    }

    return gitignore;
}

fn visit_dir(dir: &Path, ext: &str, gitignore: &mut Vec<String>) -> std::io::Result<usize> {
    let mut lines = 0usize;
    if dir.is_dir() {
        log::info!("Old gitignore passed into {:?} dir: {:?}", dir.file_name(), gitignore);
        gitignore.append(&mut get_gitignore(dir));
        log::info!("Added new Gitignore in new dir: {:?}", gitignore);
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with('.') {
                continue;
            }

            
            let contains = gitignore.iter().any(|s| s == path.file_name().unwrap().to_str().unwrap());

            if contains {
                log::info!("Ignored file: {:?}", path.file_name().unwrap());
                continue;
            }

            if path.is_dir() {
                log::info!("Dir name {:?}", path.file_name().unwrap());
                lines += visit_dir(&path, ext, gitignore)?;
            } else {
                if path.file_name().unwrap().to_str().unwrap().ends_with(ext) {
                    log::info!("Good file with good ext");
                    log::info!("Filename name {:?}", path.file_name().unwrap());
                    lines += count_lines(&path);
                } else {
                    continue;
                }
            }
            log::info!("Total amount of lines: {}\n", &lines);
        }
    }
    log::info!("Getting out of {:?}", dir.file_name());
    log::info!("Total lines in {:?}: {}",dir.file_name(), lines);
    Ok(lines)
}

fn count_lines(file: &Path) -> usize {
    let file_str = fs::read_to_string(file).unwrap();

    let new_lines_re = Regex::new(r#"\n{2,}"#).unwrap();
    let multi_comment = Regex::new(r#"/\*[\s\S]*\*/\s"#).unwrap();

    let file_str = new_lines_re.replace_all(&file_str, "\n");
    let file_str = multi_comment.replace_all(&file_str, "");
    let mut lines: Vec<&str> = file_str.lines().collect();
    for (i, line) in lines.clone().into_iter().enumerate() {
        if line.starts_with("//") {
            lines.remove(i);
        }
    }
    log::info!("Lines in {:?}: {}",file.file_name(), lines.clone().len());
    lines.len()
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        Builder::new()
            .filter(None, log::LevelFilter::Info)
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
                let lines = visit_dir(&cli.path, &cli.extension, &mut vec![]);
                println!("{}", lines.unwrap());
            }
        },
        Err(_) => ()
    }
}
