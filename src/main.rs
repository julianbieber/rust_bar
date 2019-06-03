extern crate chrono;
extern crate battery;
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
        &Command::new("xssstate").arg("-i")
            .output()
            .expect("Failed to run xssstste")
            .stdout,
    )
    .expect("Could not parse the output of getIdle")
    .to_string();
    let _ = string.pop();
    format!("{}", string.parse::<u32>().expect("Failed to parse idle state") / 60000)
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
    Ok( tasks.join(" | "))
}

fn get_battery_string(battery_manager: &battery::Manager) -> Option<String> {
    battery_manager.batteries().ok().and_then(|mut batteries| {
        batteries.next().and_then(|battery_opt| {
            match battery_opt {
                Ok(battery) => Some(format!("{:.2}", f32::from(battery.state_of_charge()))),
                _ => None
            }
        })
    })
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let battery_manager = battery::Manager::new()?;
    loop {

        let bar = vec![
            get_battery_string(&battery_manager),
            Some(idle_time()),
            get_current_tasks().ok(),
            Some(time_string())
        ].into_iter().flatten().collect::<Vec<String>>().join(" | ");
        Command::new("xsetroot")
            .arg("-name")
            .arg(bar)
            .output()
            .expect("");
        sleep(Duration::new(1, 0));
    }
}
