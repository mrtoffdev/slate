use std::str::SplitWhitespace;
use crate::models::schedule::{Bound, Regular, Schedule};
use time::{Time, Duration};

pub(crate) fn handler(line: String) {
        let mut TEMP: Schedule  = Schedule::RegularV(Regular::default());
        let mut SchedType   = 'R';

        println!("Added to Subject");
        let mut split_line: SplitWhitespace = line
                .split_whitespace();

        // Common Schedule Metadata

        // Block
        let block: String       = next_line(&mut split_line);

        // Mode
        let mode: String        = next_line(&mut split_line);

        // Room
        let room: String        = "".to_string();

        // Units
        let units: String       = next_line(&mut split_line);


        // Days [Bound/Unbound]
        let days: Vec<String>   = next_line_split(&mut
                                                          split_line, ',');

        // Check for multiple days
        if days.len() > 1 {
                SchedType       = 'B';
                TEMP            = Schedule::BoundV(Bound::default());
        }


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

        let end_split:Vec<u8>           = next_line_time
                (&mut split_line);

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
        println!("Day {}", days.concat());
        println!("Start: {}", start);
        println!("End: {}", end);
        println!("Duration: {}", end - start);

        // Iter through days
        for day in days {
                subject_holder.schedules.push(Regular {
                        subject: subject_holder.name
                                .to_owned(),
                        room:           room.to_owned(),
                        block:          block.to_owned(),
                        day:            day.to_owned(),
                        mode:           mode.to_owned(),

                        spotted:        false,
                        has_conflict:   false,

                        start:          start.to_owned(),
                        end:            end.to_owned(),
                        duration:       start - end,
                })
        }
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
