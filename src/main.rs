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
use colored::{ColoredString, Colorize};

use time::Time;
use time::Duration;

use models::subject::Subject;
use crate::core::audit::handler;
use crate::models::schedule::{Bound, Regular, Schedule};

fn main() {

        const SCHEDULE: &str = "./sch.in";

        // Build Schedule Node
        let mut subject_collection: Vec<Subject> = vec![];

        // Parse File
        if let Ok(sch_in) = open_file(SCHEDULE) {

                // Readable Section State
                let mut SECTION: bool           = false;

                // Temporary Holders
                let mut SUBJECT: Subject        = Subject::default();
                let mut PREV_BLOCK: String      = String::new();

                for (i, line) in sch_in.into_iter().flatten().enumerate() {

                        let top = subject_collection.len();

                        // ========== Section Parser ==========

                        // Header line & Section Start
                        if line.starts_with('+') {
                                SECTION = true;

                                if let Some(name) = line.strip_prefix('+') {
                                        SUBJECT.name = name.to_owned();
                                }

                                println!("Subject Found: {}", SUBJECT.name);
                                continue;
                        }

                        // End of File Line
                        if line.starts_with("---"){
                                break
                        }

                        // Section End Line
                        if line.starts_with('-') {
                                // End section
                                SECTION = false;
                                //
                                subject_collection.push(SUBJECT.clone());

                                // reset holder
                                println!("Subject pushed to collection: {}",
                                         subject_collection[top].name);
                                SUBJECT = Subject::default();
                        }

                        // Empty Line
                        if line.starts_with(' ') || line.starts_with('\n') || line
                                .is_empty(){
                                println!("whitespace found");
                                continue
                        }

                        // ====================================

                        // Non-header / terminator line found
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

                        println!();
                }
        }

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

                        }
                }
        }

        // Output Dialogue
        {

        println!("Test 1: All Schedules\n");
        for (i, entry) in subject_collection.iter().enumerate() {
                println!("{}: {}", i+1, entry.name);
        }

                                // Colored Mode Indicator
                                let mode_colored = if entry. == "Lab"
                                { entry.mode.red() }
                                else
                                { entry.mode.green() };

                                // Acronym to Full Day
                                let day_expanded = match entry.day.as_str() {
                                        "M" => "Monday",
                                        "T" => "Tuesday",
                                        "W" => "Wednesday",
                                        "Th" => "Thursday",
                                        "F" => "Friday",
                                        "S" => "Saturday",
                                        _ => "N/A"
                                };

                                println!("{: <10} {: <8} [{}]: {} - {} | {}",
                                         day_expanded,
                                         entry.block,
                                         mode_colored,
                                         entry.start,
                                         entry.end,
                                         entry.end - entry.start
                                )
                        }
                        println!()
                }
        }
        println!();

        println!("First Subject: {}", subject_collection[0].name);

}
