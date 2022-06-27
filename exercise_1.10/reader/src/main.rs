use anyhow::Result;
use hex::ToHex;
use ring::digest::{Context, Digest, SHA256};
use std::{env, fs::File, io::Read, path::PathBuf, thread::sleep, time::Duration};

fn wait_for_file(path: PathBuf) -> Result<()> {
    while !path.exists() {
        sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn main() -> Result<()> {
    let data_dir: PathBuf = env::var_os("DATA_PATH")
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("/tmp".into())
        .parse()?;

    let file_path = data_dir.join("timestamp.txt");

    wait_for_file(file_path.clone())?;
    let mut buffer = String::new();

    loop {
        {
            let mut file = File::open(&file_path)?;
            file.read_to_string(&mut buffer)?;
            let mut ctx = Context::new(&SHA256);
            ctx.update(buffer.as_bytes());
            let hash = ctx.finish().encode_hex::<String>();

            println!("{} {}", buffer, hash);

            buffer.clear();
        }

        sleep(Duration::from_secs(5));
    }
}
