#![allow(dead_code)]

use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use regex::Regex;
use std::fs::File;
use std::path::PathBuf;
use which::which;

#[derive(Debug)]
struct Cmd {
    options: Vec<String>,
    sources: Vec<String>,
    dests: Vec<String>,
}

fn read_config() {}

pub fn data_path(path_name: &str) -> std::io::Result<PathBuf> {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push(path_name);
    if !buf.as_path().exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Missing Data Path",
        ));
    }
    Ok(buf)
}

fn get_options(input_file: &str) -> Cmd {
    let pattern_rx: Regex = Regex::new(r"([\w\-]+)\s*=\s*([\w./-:@]+)\s*").unwrap();
    let file = File::open(input_file)
        .expect(format!("Unable to open configuration file: {:?}", input_file).as_str());
    let reader = BufReader::new(file);

    let mut sources = Vec::new();
    let mut dests = Vec::new();
    let mut options = Vec::new();
    options.push("-az".to_string()); // Add default options -az:

    for line in reader.lines() {
        let trimmed_line = line.unwrap().trim().to_string();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        match pattern_rx.captures(&trimmed_line) {
            Some(_m) => {
                let key = _m.get(1).unwrap().as_str();
                let value = _m.get(2).unwrap().as_str();
                let bools = ["compress", "cvs-exclude", "delete", "dry-run", "verbose"];
                let valued = ["bwlimit", "exclude", "exclude-from", "source"];

                // Separate/Define options based on key, val
                // -----------------------------------------
                // Set Source
                if key.contains("source") {
                    sources.push(value.to_string());
                // Multiple destinations required
                } else if key.contains("dest") && !value.is_empty() {
                    dests.push(value.to_string());
                // Set booleans as --key
                } else if bools.contains(&key) && ["yes", "true"].contains(&value) {
                    options.push(format!("--{}", key))
                // Set valued as --key=value
                } else if valued.contains(&key) && !value.is_empty() {
                    options.push(format!("--{}={}", key, value))
                }
            }
            None => {
                println!("{:?}", trimmed_line);
            }
        }
    }

    Cmd {
        options,
        sources,
        dests,
    }
}

pub fn run_rsync(input_file: &str) -> Result<()> {
    let rsync_path = which("rsync").context("Failed to find rsync")?;
    let options = get_options(input_file);
    let mut errors = false;

    for source in &options.sources {
        for dest in &options.dests {
            // Create a base command with shared options
            let mut command = Command::new(&rsync_path);
            for opt in &options.options {
                command.arg(opt);
            }

            // Clone the base command and add source and destination
            command.arg(source);
            command.arg(dest);

            let command_display = format!("{:?}", command).to_string().replace("\"", "");
            println!("Running: \"{}\"", command_display);

            // Set up the command to use pipes for stdout and stderr
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());

            // Run the command
            let mut child = command.spawn().context("Failed to spawn rsync command")?;

            // Read stdout in real-time
            if let Some(stdout) = child.stdout.take() {
                let stdout_reader = BufReader::new(stdout);
                for line in stdout_reader.lines() {
                    println!("{}", line.context("Failed to read stdout")?);
                }
            }

            // Read stderr in real-time
            if let Some(stderr) = child.stderr.take() {
                let stderr_reader = BufReader::new(stderr);
                for line in stderr_reader.lines() {
                    errors = true;
                    eprintln!("Error: {}", line.context("Failed to read stderr")?);
                }
            }

            // Wait for the command to finish
            let status = child
                .wait()
                .context("Failed to wait for rsync to complete")?;

            if !status.success() {
                errors = true;
                eprintln!("Command failed with exit code: {}", status);
            }
        }
    }

    if errors {
        anyhow::bail!("There were rsync errors: See transcript of output above.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Import the function and Cmd struct

    #[test]
    fn test_data_path() {
        let path = data_path("data").unwrap();
        assert!(&path.as_path().exists());
    }

    #[test]
    fn test_get_options() {
        let test_file = "data/.ryst";

        // Call the function and assert the result
        let output = get_options(test_file);
        assert!(output.sources.contains(&"./".to_string()));
        assert!(output.options.contains(&"-az".to_string()));
        assert!(!output.dests.is_empty());
        println!("actual = {:?}", output);
    }

    #[test]
    fn test_run_rsync() -> Result<()> {
        // Call the function and assert the result
        let mut path = data_path("data")?;
        path.push(".ryst");

        let path_str = path.to_str().expect("Unable to convert path to str");
        let output = run_rsync(path_str);

        // We expect both rsyncs to fail, due to non-existant hosts:
        if output.is_err() {
            assert!(true);
        } else {
            assert!(false)
        }

        if let Some(e) = output.as_ref().err() {
            println!("# ----------------------------------------------------------");
            println!("# --- Expected Errors for test_run_rsync -------------------");
            println!("# ----------------------------------------------------------");
            for line in e.to_string().trim().split('\n') {
                println!("   =>   {}", line);
            }
        }
        Ok(())
    }
}
