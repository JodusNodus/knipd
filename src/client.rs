extern crate clap;
use clap::{Arg, App};

mod shared;
use shared::*;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read};


fn list() {
    let path = get_path(LINE_CACHE);
    let file_res = OpenOptions::new()
        .read(true)
        .open(&path);
    match file_res {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            for line in contents.lines().rev() {
                println!("{}", line);
            }
        },
        Err(e) => eprintln!("Failed to open the file: {}", e),
    }
}

fn print_file(mut file: File) {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap_or(0);
    return buffer;
}

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("list")
             .short("l")
             .long("list")
             .help("Print the clipboard history")
             .takes_value(false))
        .arg(Arg::with_name("print")
             .short("p")
             .long("print")
             .help("Print content of item provided by stdin")
             .takes_value(false))
        .get_matches();

    if matches.occurrences_of("list") > 0 {
        list();
        return;
    }
    if matches.occurrences_of("print") > 0 {
        let title = read_stdin();
        match open_content_file(&title) {
            Ok(file) => print_file(file),
            Err(err) => eprintln!("Error reading history item {}:", err),
        }
        return;
    }
}

