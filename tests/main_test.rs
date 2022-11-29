extern crate core;

mod common;

use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

const SFC: &str = "safe-mac-cli";

#[test]
fn basic_case_success() -> Result<()> {
    let tmp = TempDir::new()?;
    init_config_file(&tmp, "config.txt")?;
    assert_cmd::Command::cargo_bin(SFC)
        .unwrap()
        .current_dir(tmp.path())
        .arg("config.txt")
        .assert()
        .success();
    check_result(tmp)?;
    Ok(())
}

#[test]
fn basic_case_fail() -> Result<()> {
    let tmp = TempDir::new()?;
    assert_cmd::Command::cargo_bin(SFC)
        .unwrap()
        .current_dir(tmp.path())
        .arg("config.txt")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn default_arg_success() -> Result<()> {
    let tmp = TempDir::new()?;
    init_config_file(&tmp, ".env")?;
    assert_cmd::Command::cargo_bin(SFC)
        .unwrap()
        .current_dir(tmp.path())
        .assert()
        .success();
    check_result(tmp)?;
    Ok(())
}

#[test]
fn default_arg_fail() -> Result<()> {
    let tmp = TempDir::new()?;
    assert_cmd::Command::cargo_bin(SFC)
        .unwrap()
        .current_dir(tmp.path())
        .assert()
        .failure();
    Ok(())
}

fn init_config_file(tmp: &TempDir, name: &str) -> Result<()> {
    File::create(tmp.path().join(name))?.write_all(
        "
        BOT_TOKEN=5952187433:AAElWDo96OZExms06d4zqKGvtJ81BI-DaXw
        BOT_PORT=3001
        SERVER_HOST=http://localhost:8081
        REDIS_URL=redis://127.0.0.1/
        MONGODB_HOST=mongodb://localhost:27017/Hackathon
        "
        .as_bytes(),
    )?;
    Ok(())
}

fn check_result(tmp: TempDir) -> Result<()> {
    match fs::read_to_string(tmp.path().join(".safe_env")) {
        Ok(line) => {
            assert_eq!(line.contains("MAC_ADDR"), true);
            log::info!("\n{}\n", line);
        }
        Err(err) => {
            log::error!("{}", err);
            panic!("Test failed!")
        }
    }
    Ok(())
}
