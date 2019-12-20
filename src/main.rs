extern crate chrono;
extern crate battery;
mod config;
mod percentage_clock;

use chrono::{Local, DateTime, Timelike, Utc};

use std::error::Error;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::str;
use std::thread::sleep;
use std::time::{Duration, Instant};

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
                Ok(battery) => Some(format!("battery: {:.2}", f32::from(battery.state_of_charge()))),
                _ => None
            }
        })
    })
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let battery_manager = battery::Manager::new()?;
    loop {
        let morning = Utc::now()
            .with_hour(10)
            .map(|d| d.with_minute(0)).flatten()
            .map(|d| d.with_second(0)).flatten()
            .map(|d| d.with_nanosecond(0)).flatten();

        let evening = Utc::now()
            .with_hour(18)
            .map(|d| d.with_minute(0)).flatten()
            .map(|d| d.with_second(0)).flatten()
            .map(|d| d.with_nanosecond(0)).flatten();

        let night = Utc::now()
            .with_hour(23)
            .map(|d| d.with_minute(59)).flatten()
            .map(|d| d.with_second(0)).flatten()
            .map(|d| d.with_nanosecond(0)).flatten();

        let work_clock = match (&morning, &evening) {
            (Some(start), Some(end)) => {
                Some(format!("work: {:.2} ", percentage_clock::get_current_percent_between(start, end)))
            },
            _ => None
        };

        let wake_clock = match (&morning, &night) {
            (Some(start), Some(end)) => {
                Some(format!("wake: {:.2} ", percentage_clock::get_current_percent_between(start, end)))
            },
            _ => None
        };

        let bar = vec![
            get_battery_string(&battery_manager),
            work_clock,
            wake_clock,
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
