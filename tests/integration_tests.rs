#[cfg(test)]
mod tests {

    use anyhow::Result;
    use ryst_lib::{data_path, run_rsync};

    #[test]
    fn test_run_rsync_integration() -> Result<()> {
        let current_dir = std::env::current_dir().unwrap();
        println!("Current directory: {}", current_dir.display());

        // Call the function and assert the result
        let path = data_path("tests/data/.ryst")?;

        let path_str = path.to_str().expect("Failed to convert PathBuf to str");
        let output = run_rsync(path_str);
        println!("   output =>   {:?}", output);

        if let Some(e) = output.err() {
            println!("# ----------------------------------------------------------");
            println!("# --- test_run_rsync Errors --------------------------------");
            println!("# ----------------------------------------------------------");
            for line in e.to_string().trim().split('\n') {
                println!("   =>   {}", line);
            }
            assert!(false);
        }
        Ok(())
    }
}
