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
}
