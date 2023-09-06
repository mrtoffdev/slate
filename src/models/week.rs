use std::collections::{HashMap, HashSet};
use time::Time;
use crate::models::subject::Subject;

pub(crate) enum Days {
        Monday(Day),
        Tuesday(Day),
        Wednesday(Day),
        Thursday(Day),
        Friday(Day),
        Saturday(Day)
}

pub(crate) type Week = HashSet<Days>;

pub(crate) struct Day {
        occupied: Vec<Time>,
        subjects: Vec<Subject>
}