use std::{
    time,
    thread,
    fs,
    process,
    str,
    io::{prelude::*, BufReader},
    path::Path,
    ops,
    env,
};

use exitcode;
use chrono::{Local, DateTime, Timelike, Datelike};


#[derive(Debug)]
struct Job {
    minute_filter: Option<Vec<u32>>,
    hour_filter: Option<Vec<u32>>,
    day_of_month_filter: Option<Vec<u32>>,
    month_filter: Option<Vec<u32>>,
    day_of_week_filter: Option<Vec<u32>>,
    command: String,
}

impl Job {
    fn new() -> Self{
        return Job{
            minute_filter: None,
            hour_filter: None,
            day_of_month_filter: None,
            month_filter: None,
            day_of_week_filter: None,
            command: String::new(),
        }
    }

    fn from_string(string: &str) -> Result<Self, String> {
        let mut job = Job::new();

        let str_list = string.split_whitespace();
        for (i, val) in str_list.enumerate() {
            match i {
                0 => job.minute_filter = {
                    if val == "*" {
                        None
                    } else {
                        match parse_to_list(val, &(0..60)) {
                            Ok(val) => Some(val),
                            Err(e) => return Err(e),
                        }
                    }
                },
                1 => job.hour_filter = {
                    if val == "*" {
                        None
                    } else {
                        match parse_to_list(val, &(0..24)) {
                            Ok(val) => Some(val),
                            Err(e) => return Err(e),
                        }
                    }
                },
                2 => job.day_of_month_filter = {
                    if val == "*" {
                        None
                    } else {
                        match parse_to_list(val, &(1..32)) {
                            Ok(val) => Some(val),
                            Err(e) => return Err(e),
                        }
                    }
                },
                3 => job.month_filter = {
                    if val == "*" {
                        None
                    } else {
                        match parse_to_list(val, &(1..13)) {
                            Ok(val) => Some(val),
                            Err(e) => return Err(e),
                        }
                    }
                },
                4 => job.day_of_week_filter = {
                    if val == "*" {
                        None
                    } else {
                        match parse_to_list(val, &(0..8)) {
                            Ok(val) => {
                                let mut values = val.clone();
                                if values.contains(&0) {
                                    values.push(7);
                                }
                                Some(values)
                            },
                            Err(e) => return Err(e),
                        }
                    }
                },
                _ => {
                    if job.command.is_empty() {
                        job.command.push_str(val);
                    } else {
                        job.command.push_str(&format!(" {}", val));
                    }
                },
            }
        }

        if job.command.trim().is_empty() {
            return Err(String::from("Failed to create job with empty command"));
        }

        Ok(job)

    }

    fn check_time(&self, time: Option<DateTime<Local>>) -> bool
    {
        let local_time: DateTime<Local> = match time{
            Some(val) => val,
            None => Local::now(),
        };

        if let Some(month) = &self.month_filter {
            if !month.contains(&local_time.month()) {
                return false;
            }
        }

        if let Some(day_of_month) = &self.day_of_month_filter {
            if !day_of_month.contains(&local_time.day()) {
                return false;
            }
        }

        if let Some(day_of_week) = &self.day_of_week_filter {
            if !day_of_week.contains(
                &(local_time.weekday().num_days_from_monday() + 1)) {
                return false;
            }
        }

        if let Some(hour) = &self.hour_filter {
            if !hour.contains(&local_time.hour()) {
                return false;
            }
        }

        if let Some(minute) = &self.minute_filter {
            if !minute.contains(&local_time.minute()) {
                return false;
            }
        }

        true
    }

    fn run(&self) {
        let mut command = process::Command::new("bash");
        command.arg("-c");
        command.arg(self.command.clone());

        match command.spawn() {
            Ok(child) => println!("Process ID is {}", child.id()),
            Err(e) => println!("Failed to spawn process: {e}"),
        };
    }
}

fn parse_range(section: &str, range: &ops::Range<u32>) -> Result<Vec<u32>, String> {
    let Some(range_delimiter_index) =  section.find("-") else {
       return Err(format!("No delimiter '-' found in {section}"));
    };

    let mut current_range = range.clone();

    if range_delimiter_index == 0 {
        current_range.start = match section[range_delimiter_index + 1..].parse() {
            Ok(val) => val,
            Err(e) => return Err(format!(
                "Failed to parse range section {section}: {}", 
                e.to_string())
            ),
        }
    } else if range_delimiter_index == section.len() - 1 {
        current_range.end = match section[..range_delimiter_index].parse() {
            Ok(val) => val,
            Err(e) => return Err(format!(
                "Failed to parse range section {section}: {}", 
                e.to_string())
            ),
        };
    } else {
        current_range.start = match section[..range_delimiter_index].parse() {
            Ok(val) => val,
            Err(e) => return Err(format!(
                "Failed to parse range section {section}: {}", 
                e.to_string())
            ),
        };

        current_range.end = match section[range_delimiter_index + 1..].parse() {
            Ok(val) => val,
            Err(e) => return Err(format!(
                "Failed to parse range section {section}: {}",
                e.to_string())
            ),
        };
    }

    if !range.contains(&current_range.start)
            || !range.contains(&current_range.end) {
        return Err(format!(
            "Requested range [{}, {}] is not in the expected range [{}, {}[",
            current_range.start,
            current_range.end,
            range.start,
            range.end)
        );
    }

    Ok(current_range.collect())
}


fn parse_to_list(string: &str, range: &ops::Range<u32>) -> Result<Vec<u32>, String> {
    let mut values: Vec<u32> = Vec::new();

    let parts = string.split(",");

    for section in parts {
        let mut modulator: Option<usize> = None;
        let mut section_span = &section[..];
        if let Some(modulator_index) = section.find("/"){
            section_span = &section[..modulator_index];
            modulator = match section[modulator_index+1..].parse() {
                Ok(res) => Some(res),
                Err(_) => return Err(
                    format!("failed to extract modulator after '/' in {section}")),
            };
        }

        if section_span.contains("-") {
            let mut section_values = parse_range(section_span, &range)?;

            if let Some(step_size) = modulator {
                section_values = section_values.iter()
                    .skip(step_size-1)
                    .step_by(step_size)
                    .copied()
                    .collect();
            }
            
            values.append(&mut section_values);

        } else if section_span == "*" {
            let mut section_values: Vec<u32> = range.clone().collect();

            if let Some(step_size) = modulator {
                section_values = section_values.iter()
                    .skip(step_size-1)
                    .step_by(step_size)
                    .copied()
                    .collect();
            }
            
            values.append(&mut section_values);

        } else {
            let value: u32 = match section.parse() {
                Ok(val) => val,
                Err(e) => return Err(
                    format!("Failed to parse range section {section}: {}", e.to_string())
                ),
            };
            if !range.contains(&value) {
                return Err(format!("Value {value} is not in the expected range [{}, {}[",
                    range.start,
                    range.end)
                );
            }

            values.push(value);
        }
    }

    Ok(values)
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name;

    match args.len() {
        1 => {
            println!("Pass a filename as argument: docker-cron <filename>");
            process::exit(exitcode::USAGE);
        },
        2 => {
            file_name = Path::new(&args[1]);
            println!("Using filename {}", &args[1]);
        },
        _ => {
            println!("More than one argument is not allowed. Usage: docker-cron <filename>");
            process::exit(exitcode::USAGE);
        },
    }

    let lines = lines_from_file(file_name);
    let mut jobs: Vec<Job> = Vec::new();

    for line in lines {
        let command_line;
        if let Some(index) = line.find("#") {
            command_line = &line[..index];
        } else {
            command_line = &line;
        }

        let job = match Job::from_string(command_line) {
            Ok(val) => val,
            Err(e) => {
                println!("Failed to create job: {}", e);
                continue;
            },
        };

        println!("Added {:?}", job);
        jobs.push(job);
    }

    loop {
        let time = Local::now();

        for job in &jobs {
            if job.check_time(Some(time)) {
                job.run();
            }
        }

        thread::sleep(time::Duration::from_secs(60));
    }
}
