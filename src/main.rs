extern crate clap;

use std::collections::hash_map::DefaultHasher;
use std::fs::{copy, File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::Write;

use clap::{App, Arg};
#[allow(unused_imports)]
use ctor::ctor;
use log::LevelFilter;
use mac_address::get_mac_address;
use simple_logger::SimpleLogger;

#[ctor::ctor]
fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
}

fn main(){
    let safe = self_mac_addr_as_string();
    let senv = ".safe_env";
    let matches = App::new("safe-mac-cli")
        .author("Ivan I. <clicker.heroes.acg@gmail.com")
        .version("0.1.0")
        .name("safe-mac-cli")
        .about("Encrypts mac address and add into your .env file.")
    .arg(Arg::with_name("dotenv")
        .long("dotenv")
        .short("de")
        .default_value(".env")
        .help("Your .env filename(default=\".env\")")
        .takes_value(true)
        .required(false)
        .index(1))
        .get_matches();
    let dotenv = matches.value_of("dotenv").unwrap();
    File::create(senv).expect("Creation failed...");
    copy(dotenv, senv)
        .expect("Error: can't create a copy of data...");
    let mut cfg = OpenOptions::new()
        .write(true)
        .append(true)
        .open(senv)
        .expect("Error: can't open a copy...");
    write!(cfg, "{}", safe).and_then(|()| {
        log::info!("Done! You can take a look at \".safe_env\" file!");
        Ok(())
    }).expect("Error: can't create a copy...");
}

fn self_mac_addr_as_string() -> String {
    let addr_as_bytes = get_mac_address().unwrap().unwrap().bytes();
    let addr = pnet::util::MacAddr::from(addr_as_bytes);
    format!("MAC_ADDR={}", hash(&addr))
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}