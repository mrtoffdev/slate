#[derive(Default, Clone, PartialEq)]
struct Schedule {
        // Subject Metadata
        subject:        String,
        room:           String,
        spotted:        bool,
        has_conflict:   bool,

        // Time Metadata
        start:          u16,
        end:            u16,
        duration:       u16,
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

fn main() {
    println!("Hello, world!");

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
                        println!("\n")
                }
        }


        println!("First Subject: {}", subject_collection[0].name);

        println!("Schedule Management:");
}
