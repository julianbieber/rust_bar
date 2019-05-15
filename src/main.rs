extern crate chrono;

mod config;

use chrono::Local;

use std::error::Error;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::str;
use std::thread::sleep;
use std::time::Duration;

fn time_string() -> String {
    let now = Local::now();
    format!("{}", now.format("(%H:%M) (%d-%m-%Y)"))
}

fn idle_time() -> String {
    let mut string = str::from_utf8(
        &Command::new("getIdle")
            .output()
            .expect("Failed to run getIdle")
            .stdout,
    )
    .expect("Could not parse the output of getIdle")
    .to_string();
    let _ = string.pop();
    string
}

fn get_current_tasks() -> std::result::Result<String, Box<dyn Error>> {
    let mut tasks: Vec<String> = Vec::new();
    for file_result in read_dir(config::base_path())? {
        let file = file_result?;
        if file.path().is_file() {
            let reader = BufReader::new(OpenOptions::new().read(true).open(file.path())?);
            if reader.lines().count() % 2 == 1 {
                tasks.push(file.file_name().into_string().expect(""))
            }
        }
    }
    Ok(tasks.join(" | "))
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    loop {
        Command::new("xsetroot")
            .arg("-name")
            .arg(format!(
                "{} | {} {}",
                idle_time(),
                get_current_tasks()?,
                time_string()
            ))
            .output()
            .expect("");
        sleep(Duration::new(1, 0));
    }
}
