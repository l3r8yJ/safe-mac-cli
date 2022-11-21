extern crate clap;

use std::{env, fs};
use std::cell::Ref;
use std::collections::hash_map::DefaultHasher;
use std::fmt::format;
use std::fs::{copy, File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Lines, LineWriter, Write};
use std::path::Path;

use clap::App;
use mac_address::get_mac_address;

fn main(){
    let safe = self_mac_addr_as_string();
    copy("src/config.txt", ".env")
        .expect("Error: can't create a copy...");
    let mut cfg = OpenOptions::new()
        .write(true)
        .append(true)
        .open(".env")
        .expect("Error: can't open copy...");
    write!(cfg, "{}", safe)
        .expect("Error: can't append new line...");
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