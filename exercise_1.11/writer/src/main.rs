use anyhow::Result;
use chrono::Utc;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let data_dir: PathBuf = env::var_os("DATA_PATH")
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("/tmp".into())
        .parse()?;

    let file_path = data_dir.join("timestamp.txt");

    loop {
        {
            let mut file = File::create(&file_path)?;
            let timestamp = Utc::now();
            file.write(timestamp.to_string().as_bytes())?;
        }

        sleep(Duration::from_secs(5));
    }
}
