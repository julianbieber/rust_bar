extern crate chrono;

mod config;

use chrono::Local;

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufReader, BufRead};
use std::error::Error;

fn time_string() -> String {
    let now = Local::now();
    format!("{}", now.format("(%H:%M) (%d-%m-%Y)"))
}

fn get_current_tasks() -> std::result::Result<String, Box<dyn Error>> {
    let mut tasks: Vec<String> = Vec::new();
    for file_result in read_dir(config::base_path())? {
        let file = file_result?;
        if file.path().is_file() {
            let reader = BufReader::new(OpenOptions::new().read(true).open(file.path())?);
            if reader.lines().count() % 2 == 1{
                tasks.push(file.file_name().into_string().expect(""))
            }
        }
    }
    Ok(tasks.join(" | "))
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    loop {
        Command::new("xsetroot").arg("-name").arg(format!("{} {}", get_current_tasks()?, time_string())).output().expect("");
        sleep(Duration::new(1, 0));
    }
}
