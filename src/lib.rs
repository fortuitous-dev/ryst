#![allow(dead_code)]

use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use which::which;

#[derive(Debug)]
struct Cmd {
    options: Vec<String>,
    source: String,
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

    let mut source = String::new();
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
                    source = value.to_string();
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
        source,
        dests,
    }
}

pub fn run_rsync(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rsync_path = which("rsync").map_err(|e| format!("Failed to find rsync: {}", e))?;
    let options = get_options(input_file);
    let mut errors = false;

    for dest in options.dests {
        // Construct and display the command to be run
        let mut command = Command::new(&rsync_path);
        for opt in &options.options {
            command.arg(opt);
        }
        command.arg(&options.source);
        command.arg(&dest);
        let command_display = format!("{:?}", command).to_string().replace("\"", "");
        println!("Running: \"{}\"", command_display);

        // Run the command
        let output = command.output()?;

        // Print the output in real-time
        let out_str = String::from_utf8_lossy(&output.stdout);
        if !out_str.is_empty() {
            println!("Output:\n{}", out_str);
        }

        // Print the error in real-time
        let err_str = String::from_utf8_lossy(&output.stderr);
        if !err_str.is_empty() {
            errors = true;
            eprintln!("{}", "=".repeat(82));
            eprintln!("Errors:");
            for line in err_str.lines() {
                eprintln!("\t{}", line);
            }
            eprintln!("{}", "=".repeat(82));
        }
    }
    if errors {
        // Err(format!("{}", 9))?
        Err(Box::new(Error::new(
            std::io::ErrorKind::Other,
            "There were rsync errors: See transcript of output above.",
        )))?
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
        assert!(output.source.contains("./"));
        assert!(output.options.contains(&"-az".to_string()));
        assert!(!output.dests.is_empty());
        println!("actual = {:?}", output);
    }

    #[test]
    fn test_run_rsync() -> Result<(), Box<dyn std::error::Error>> {
        // Call the function and assert the result
        let mut path = data_path("data")?;
        path.push(".ryst");

        let path_str = path.to_str().ok_or("Failed to convert PathBuf to str")?;
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
