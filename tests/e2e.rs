#[cfg(test)]
mod tests {
    use std::{
        io::{BufReader, Read},
        process::{Command, Stdio},
    };

    #[test]
    fn without_args() {
        let output = Command::new("cargo").arg("run").output().expect("");
        let str = String::from_utf8_lossy(&output.stdout);
        let _num: usize = str.trim().parse().expect("Should be valid number");
    }

    #[test]
    fn with_path() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());

        // Will check only .rs files
        let num: usize = output.trim().parse().expect("Should be valid number");
        assert_eq!(num, 10);
    }

    // From here app will be tested with files
    // in mock_files dir
    
    #[test]
    fn with_all_exts() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .arg("-e")
            .arg(".rs")
            .arg(".js")
            .arg(".c")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());

        let num: usize = output.trim().parse().expect("Should be valid number");
        assert_eq!(num, 46);
    }

    #[test]
    fn with_all_exts_and_comments() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .arg("-e")
            .arg(".rs")
            .arg(".js")
            .arg(".c")
            .arg("-c")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());

        let num: usize = output.trim().parse().expect("Should be valid number");
        assert_eq!(num, 66);
    }

    #[test]
    fn with_all_exts_and_comments_and_docs() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .arg("-e")
            .arg(".rs")
            .arg(".js")
            .arg(".c")
            .arg("-cd")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());

        let num: usize = output.trim().parse().expect("Should be valid number");
        assert_eq!(num, 71);
    }

    #[test]
    fn with_hidden_only_rust() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .arg("-a")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());

        let num: usize = output.trim().parse().expect("Should be valid number");
        assert_eq!(num, 16);
    }

    #[test]
    fn with_ratio_only_rust_docs_and_comments() {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("-p")
            .arg("./mock_files")
            .arg("-rdc")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("");

        let mut output = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            reader
                .read_to_string(&mut output)
                .expect("failed to read stdout");
        }

        let status = child.wait().expect("failed to wait on child process");
        assert!(status.success());
        let expected_str = String::from("20\ncomments: 25.0%\ndocs: 25.0%\nloc: 50.0%\n");

        assert_eq!(output, expected_str);
    }
}
