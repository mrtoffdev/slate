#[derive(Clone, PartialEq)]
struct Schedule {
        // Subject Metadata
        subject:        String,
        room:           String,
        spotted:        bool,
        has_conflict:   bool,

        // Time Metadata
        start:          Time,
        end:            Time,
        duration:       Duration,
}

impl Default for Schedule {
        fn default() -> Self {
                Schedule {
                        subject:        "N/A".to_string(),
                        room:           "".to_string(),
                        spotted:        false,
                        has_conflict:   false,
                        start:          Time::from_hms(0,0,0).unwrap(),
                        end:            Time::from_hms(0,0,0).unwrap(),
                        duration:       Duration::default(),
                }
        }
}

#[derive(Default, Clone, PartialEq)]
struct Subject {
        name:           String,
        prof:           String,
        schedules:      Vec<Schedule>,
}

/*
        Schedule Management Process
        1. Fetch subjects to take
        2. Fetch subject schedules for all courses, blocks, etc.
        3. Check against existing std schedule
        4. If conflicts are found: let overlapping subjects be A & B, assigned
        chronologically:
                A. Check other times available for the subject A within the week.

                        Criterion:
                        II. Subjects that have laboratory assignments should be of the
                        same block / course.
                        III. If subject A cannot find any suitable replacement
                        schedules within the week, find alternative schedules for
                        subject B instead.
                        IV. Repeat I - III to subject B
*/

use std::fs::File;
use std::vec::Vec;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::SplitWhitespace;

use time::Time;
use time::Duration;

fn main() {

        const SCHEDULE: &str = "./sch.in";
        fn open_file<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
                Ok(io::BufReader::new(File::open(file)?).lines())

        }

        // Build Schedule Node
        let mut subject_collection: Vec<Subject> = vec![];
        if let Ok(sch_in) = open_file(SCHEDULE) {
                let mut SECTION = false;
                let mut subject_holder  = Subject {
                        name: "".to_string(),
                        prof: "".to_string(),
                        schedules: vec![],
                };

                for (i, line) in sch_in.into_iter().flatten().enumerate() {

                        let top = subject_collection.len();
                        // println!("{i}: {line}");
                        if line.starts_with('+') {
                                SECTION = true;

                                if let Some(name) = line.strip_prefix('+') {
                                        subject_holder.name = name.to_owned();
                                }
                                continue;
                        }

                        if line.starts_with('-') {
                                SECTION = false;
                                subject_collection.push(subject_holder.clone());
                                println!("Subject Dispatched: {}",
                                        subject_collection[top].name);
                                // reset holder
                                subject_holder = Subject::default();
                        }

                        if line.starts_with(' ') || line.starts_with('\n') || line
                                .is_empty(){
                                println!("whitespace found");
                                continue
                        }

                        if SECTION {
                                println!("Added to Subject");
                                let mut split_line: SplitWhitespace = line
                                                                .split_whitespace();

                                let block: String               = split_line
                                                                .next()
                                                                .unwrap_or("N/A")
                                                                .to_owned();
                                let mode: String                = split_line
                                                                .next()
                                                                .unwrap_or("N/A")
                                                                .to_owned();
                                let units: String               = split_line
                                                                .next()
                                                                .unwrap_or("0")
                                                                .to_owned();

                                let day: Vec<&str>              = split_line
                                                                .next()
                                                                .unwrap_or("x")
                                                                .split(',')
                                                                .collect();

                                // Time
                                let start_split:Vec<u8>         = split_line
                                                                .next()
                                                                .unwrap_or("N/A")
                                                                .split(':')
                                                                .map(|x|
                                                                        x.parse::<u8>()
                                                                        .unwrap())
                                                                .collect();

                                let mut start_hr:u8             = start_split[0];
                                let start_mn:u8                 = start_split[1];
                                let start_sc:u8                 = 0;

                                let start_half:String           = split_line
                                                                .next()
                                                                .unwrap_or("N/A")
                                                                .to_owned();
                                println!("Start Half: {start_half}");

                                if start_half == "PM" && start_hr != 12 {
                                        start_hr += 12;
                                }

                                let start: Time                 = Time::from_hms(
                                                                start_hr,
                                                                start_mn,
                                                                start_sc
                                                                ).unwrap();

                                let end_split:Vec<u8>           = split_line
                                                                .next()
                                                                .unwrap_or("N/A")
                                                                .split(':')
                                                                .map(|x|
                                                                        x.parse::<u8>()
                                                                        .unwrap())
                                                                .collect();

                                let mut end_hr:u8               = end_split[0];
                                let end_mn:u8                   = end_split[1];
                                let end_sc:u8                   = 0;

                                let end_half:String           = split_line
                                        .next()
                                        .unwrap_or("N/A")
                                        .to_owned();

                                if end_half == "PM" && end_hr != 12 {
                                        end_hr += 12;
                                }

                                let end: Time                   = Time::from_hms(
                                        end_hr,
                                        end_mn,
                                        end_sc
                                ).unwrap();

                                // Dialogue
                                println!("Block {}", block);
                                println!("Mode {}", mode);
                                println!("Units {}", units);
                                println!("Day {}", day.concat());
                                println!("Start: {}", start);
                                println!("End: {}", end);

                                subject_holder.schedules.push(Schedule {
                                        subject: subject_holder.name
                                                .to_owned(),
                                        room: "".to_string(),
                                        spotted: false,
                                        has_conflict: false,
                                        start,
                                        end,
                                        duration: start - end,
                                })
                        }
                        println!("\n")
                }
        }


        println!("First Subject: {}", subject_collection[0].name);

        println!("Schedule Management:");
}
