use std::{fs, path::Path};

use crate::{code_stats::CodeStats, params::Params};

pub fn count_lines(path: &Path, params: &Params, stats: &mut CodeStats) {
    let file_str = fs::read_to_string(path).unwrap();

    let mut lines = file_str.lines().collect::<Vec<&str>>();

    let mut i = 0;
    let mut in_multi_comment = false;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            if params.ratio {
                stats.add_empty_lines();
            }

            lines.remove(i);
            continue;
        }

        if !params.comments {
            if line.len() < 2 && !in_multi_comment {
                i += 1;
                continue;
            }

            if in_multi_comment {
                if line.len() >= 2 && line[line.len() - 2..=line.len() - 1] == *"*/" {
                    in_multi_comment = false;
                }

                if lines[i].contains("TODO") {
                    stats.add_todo();
                }
                if lines[i].contains("FIXME") {
                    stats.add_fixme();
                }

                if params.ratio {
                    stats.add_comments();
                }

                lines.remove(i);

                continue;
            }

            if line[0..=1] == *"/*" {
                if lines[i].contains("TODO") {
                    stats.add_todo();
                }
                if lines[i].contains("FIXME") {
                    stats.add_fixme();
                }

                if params.ratio {
                    stats.add_comments();
                }

                lines.remove(i);
                in_multi_comment = true;
                continue;
            }

            if line[0..=1] == *"//" && line.chars().nth(2) != Some('/') {
                if lines[i].contains("TODO") {
                    stats.add_todo();
                }
                if lines[i].contains("FIXME") {
                    stats.add_fixme();
                }

                if params.ratio {
                    stats.add_comments();
                }

                lines.remove(i);
                continue;
            }
        }

        if !params.docs {
            if line.len() >= 3 && (line[0..=2] == *"///" || line[0..=2] == *"//!") {
                if lines[i].contains("TODO") {
                    stats.add_todo();
                }
                if lines[i].contains("FIXME") {
                    stats.add_fixme();
                }

                if params.ratio {
                    stats.add_docs();
                }

                lines.remove(i);
                continue;
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
