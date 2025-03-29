use std::{fs, path::Path};

use crate::{code_stats::CodeStats, params::Params};

pub fn count_lines(path: &Path, params: &Params, stats: &mut CodeStats) {
    let file_str = fs::read_to_string(path).unwrap();

    let mut lines = file_str.lines().collect::<Vec<&str>>();

    let mut i = 0;
    let mut in_multi_comment = false;
    while i < lines.len() {
        let line = lines[i].trim();
        let mut chars = line.chars();
        let first_char = chars.next();
        let second_char = chars.next();
        let third_char = chars.next();

        if line.is_empty() {
            lines.remove(i);
            continue;
        }

        if in_multi_comment {
            let mut reverse_chars = line.chars().rev();
            if reverse_chars.next() == Some('/') && reverse_chars.next() == Some('*') {
                in_multi_comment = false;
            }

            if line.contains("TODO") {
                stats.add_todo();
            }
            if line.contains("FIXME") {
                stats.add_fixme();
            }

            if !params.comments {
                lines.remove(i);
                continue;
            } else {
                if params.ratio {
                    stats.add_comments();
                }
            }
        }

        if first_char == Some('/') && second_char == Some('*') {
            if line.contains("TODO") {
                stats.add_todo();
            }
            if line.contains("FIXME") {
                stats.add_fixme();
            }

            in_multi_comment = true;

            if !params.comments {
                lines.remove(i);
                continue;
            } else {
                if params.ratio {
                    stats.add_comments();
                }
            }
        }

        if first_char == Some('/')
            && second_char == Some('/')
            && (third_char != Some('/') && third_char != Some('!'))
        {
            if line.contains("TODO") {
                stats.add_todo();
            }
            if line.contains("FIXME") {
                stats.add_fixme();
            }

            if !params.comments {
                lines.remove(i);
                continue;
            } else {
                if params.ratio {
                    stats.add_comments();
                }
            }
        }

        if first_char == Some('/')
            && second_char == Some('/')
            && (third_char == Some('/') || third_char == Some('!'))
        {
            // println!("{:#?}", stats);
            if lines[i].contains("TODO") {
                stats.add_todo();
            }
            if lines[i].contains("FIXME") {
                stats.add_fixme();
            }

            if !params.docs {
                lines.remove(i);
                continue;
            } else {
                if params.ratio {
                    stats.add_docs();
                }
            }
        }

        if params.units {
            if line.starts_with("struct ") || line.starts_with("pub struct ") {
                stats.add_structs();
            }
            if line.starts_with("fn ")
                || line.starts_with("async fn ")
                || line.starts_with("pub fn ")
                || line.starts_with("pub async fn ")
            {
                stats.add_fns();
            }
            if line.starts_with("impl ") {
                stats.add_impls();
            }
            if line.starts_with("macro_rules!") {
                stats.add_macros();
            }
        }

        i += 1;
    }

    // println!("{}", lines.join("\n"));

    log::info!("Lines in {:?}: {}", params.path.file_name(), lines.len());
    stats.add_loc(lines.len());
}
