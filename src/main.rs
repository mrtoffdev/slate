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
pub mod models {
        pub mod schedule;
        pub mod subject;
        pub mod week;
}
pub mod core {
        pub mod parser;
        pub mod audit;
}

use std::fs::File;
use std::vec::Vec;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::SplitWhitespace;

use time::Time;
use time::Duration;

use models::subject::Subject;
use crate::core::audit::handler;
use crate::models::schedule::{Bound, Regular, Schedule};

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

                        if line.starts_with("---"){
                                break
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

                        fn next_line(split: &mut SplitWhitespace) -> String {
                                split
                                .next()
                                .unwrap_or("N/A")
                                .to_owned()
                        }

                        fn next_line_split(split: &mut SplitWhitespace,
                                           prefix: char) -> Vec<String> {
                                split
                                .next()
                                .unwrap_or("N/A")
                                .split(prefix)
                                .map(str::to_owned)
                                .collect()
                        }

                        fn next_line_time(split: &mut SplitWhitespace) -> Vec<u8> {
                                split
                                .next()
                                .unwrap_or("0")
                                .split(':')
                                .map(|x|x.parse::<u8>().unwrap())
                                .collect()
                        }

                        if SECTION {
                                println!("Added to Subject");
                                let mut split_line: SplitWhitespace = line
                                                                .split_whitespace();

                                // Common Schedule Metadata
                                let block: String       = next_line(&mut split_line);
                                let mode: String        = next_line(&mut split_line);
                                let units: String       = next_line(&mut split_line);
                                let day: Vec<String>    = next_line_split(&mut
                                                                split_line, ',');

                                // Start Time
                                let start_split:Vec<u8> = next_line_time(&mut
                                                                split_line);

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
                                println!("Duration: {}", end - start);

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

        println!("Test 1: All Schedules\n");
        for (i, entry) in subject_collection.iter().enumerate() {
                println!("{}: {}", i+1, entry.name);
        }

        println!();

        println!("First Subject: {}", subject_collection[0].name);

        println!("Schedule Management:");
}
