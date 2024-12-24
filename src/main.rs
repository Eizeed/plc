use std::{fs, path::{Path, PathBuf}};

mod some_file;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    path: PathBuf,
    extension: String,
}

fn get_gitignore(dir: &Path) -> Result<Vec<String>, String> {
    let mut gitignore: Vec<String> = vec![];
    if dir.is_dir() {
        'outer: for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                let mut temp = get_gitignore(&path).unwrap();
                if !temp.is_empty() {
                    gitignore.append(temp.as_mut());
                }
            }

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

    return Ok(gitignore);
}

fn visit_dir(dir: &Path, ext: &str, gitignore: &Vec<String>) -> std::io::Result<usize> {
    let mut lines = 0usize;
    println!("{:?}", gitignore);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            println!("{:?}", &path.file_name());
            println!("{:?}", &lines);

            if path.file_name().unwrap_or_default().to_str().unwrap_or("").starts_with('.') {
                continue;
            }

            
            let contains = gitignore.iter().any(|s| s == path.file_name().unwrap().to_str().unwrap());

            if contains {
                println!("Ignored file: {:?}", path.file_name().unwrap());
                continue;
            }

            if path.is_dir() {
                lines += visit_dir(&path, ext, &gitignore)?;
            } else {
                if path.file_name().unwrap().to_str().unwrap().ends_with(ext) {
                    println!("Good file with good ext");
                    println!("File name {:?}", path.file_name().unwrap());
                    let temp = count_lines(&path);
                    println!("{}", temp);
                    lines += temp;
                    println!("{}", lines);
                } else {
                    continue;
                }
            }
        }
    }
    println!("Getting out of {:?}", dir.file_name());
    println!("{}", lines);
    Ok(lines)
}

fn count_lines(file: &Path) -> usize {
    let file_str = fs::read_to_string(file).unwrap();

    let regex = Regex::new(r#"\n{2,}"#).unwrap();
    let file_str = regex.replace_all(&file_str, "\n");
    println!("{}", file_str);
    let lines = file_str.lines();
    println!("{}", lines.clone().count());
    lines.count()
}

fn main() {
    let cli = Cli::parse();

    match fs::metadata(&cli.path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                let gitignore = get_gitignore(&cli.path).unwrap();
                let lines = visit_dir(&cli.path, &cli.extension, &gitignore);
                println!("{}", lines.unwrap());
            }
        },
        Err(_) => ()
    }

    println!("{:?}, {:?}",cli.path, cli.extension)
}

