use crate::models::schedule::{Regular, Schedule};
#[derive(Default, Clone, PartialEq)]
pub(crate) struct Subject {
        pub(crate) name:           String,
        pub(crate) prof:           String,
        pub(crate) schedules:      Vec<Schedule>,
}
