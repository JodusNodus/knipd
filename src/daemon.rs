extern crate chrono;
mod shared;
use shared::*;

use std::fs::File;
use std::{thread, time};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use chrono::prelude::*;

fn get_time_str() -> String {
    let dt: DateTime<Local> = Local::now();
    format!("{:?} {}:{}", dt.weekday(), dt.hour(), dt.minute())
}

fn get_linecount_str(copy_content: &String) -> String {
    let line_count = copy_content.lines().count();
    if line_count > 1 {
        format!(" ({} lines)", line_count)
    } else {
        String::from("")
    }
}

fn get_content_str(copy_content: &String) -> String {
    let first_line: &str = copy_content.lines().next().unwrap().trim();
    let clean_content: String = String::from(first_line);
    let content_slice: String = clean_content.chars().take(STR_LEN).collect();
    let tail = if clean_content.chars().count() > STR_LEN {
        "..."
    } else {
        ""
    };
    format!("{}{}", content_slice, tail)
}

fn get_cache_line(copy_content: &String) -> String {
    format!("[{}] \"{}\"{}", get_time_str(), get_content_str(&copy_content), get_linecount_str(&copy_content))
}

fn run(mut file: File) {
    let timeout = time::Duration::from_millis(1000);
    let mut last_clip_hash = 0u64;
    let mut command = Command::new("/usr/bin/pbpaste");
    command.env_clear();

    loop {
        thread::sleep(timeout);

        match command.output() {
            Err(_) => continue,
            Ok(output) => {
                let clip_hash = hash(&output.stdout);
                if output.stdout.len() < 1 || &clip_hash == &last_clip_hash {
                    continue;
                }
                last_clip_hash = clip_hash;
                match String::from_utf8(output.stdout) {
                    Err(_) => continue,
                    Ok(clip_content) => {
                        let first_line = get_cache_line(&clip_content);
                        let mut content_file = open_content_file(&first_line).unwrap();

                        content_file.set_len(0).unwrap();
                        write!(content_file, "{}", clip_content);
                        writeln!(file, "{}", first_line);
                    }
                }
            }
        }
    }
}

fn main() {
    let path = get_path(LINE_CACHE);
    let file_res = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);
    match file_res {
        Ok(file) => run(file),
        Err(e) => eprintln!("Failed to open the file: {}", e),
    }
}
