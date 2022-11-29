extern crate core;

mod common;

use std::fs;
use std::fs::File;
use std::io::{Write};
use tempfile::TempDir;
use anyhow::Result;

#[test]
fn basic_case() -> Result<()> { let tmp = TempDir::new()?;
    File::create(tmp.path().join("config.txt"))?.write_all(
        "
            BOT_TOKEN=5952187433:AAElWDo96OZExms06d4zqKGvtJ81BI-DaXw
            BOT_PORT=3001
            SERVER_HOST=http://localhost:8081
            REDIS_URL=redis://127.0.0.1/
            MONGODB_HOST=mongodb://localhost:27017/Hackathon
        "
            .as_bytes()
    )?;
    assert_cmd::Command::cargo_bin("safe-mac-cli")
        .unwrap()
        .current_dir(tmp.path())
        .arg("config.txt")
        .assert()
        .success()
        .stdout("Done! You can take a look at \".safe_env\" file!\n");
    match fs::read_to_string(tmp.path().join(".safe_env")) {
        Ok(line) => {
            assert_eq!(
                line.contains("MAC_ADDR"),
                true
            )
        }
        Err(err) => {
            log::error!("{}", err);
            panic!("Test failed!")
        }
    }
    Ok(())
}