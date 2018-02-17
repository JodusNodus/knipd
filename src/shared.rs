use std::fs::File;
use std::env;
use std::io;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub const NAME: &str = "knipd";
pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "Thomas Billiet (JodusNodus)";
pub const ABOUT: &str = "Clipboard History Daemon and CLI for macOs and X11";

pub const TMPDIR: &str = "TMPDIR";
pub const LINE_CACHE: &str = "line_cache";
pub const CACHE_LEN: u64 = 100;
pub const STR_LEN: usize = 30;

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn get_path(filename: &str) -> PathBuf {
    let tmp_dir = env::var(TMPDIR).unwrap();
    let mut path = PathBuf::from(&tmp_dir);
    path.push(format!("{}-{}-{}", NAME, VERSION, filename));
    return path;
}

pub fn open_content_file(first_line: &String) -> io::Result<File> {
    let filename = hash(first_line).to_string();
    let path = get_path(&*filename);
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
}

