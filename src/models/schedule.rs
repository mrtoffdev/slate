use time::{Time, Duration};
#[derive(Clone, PartialEq)]
pub(crate) struct Schedule {
        // Subject Metadata
        pub(crate) subject:             String,
        pub(crate) room:                String,
        pub(crate) spotted:             bool,
        pub(crate) has_conflict:        bool,

        // Time Metadata
        pub(crate) day:                 String,
        pub(crate) start:               Time,
        pub(crate) end:                 Time,
        pub(crate) duration:            Duration,
}

impl Default for Schedule {
        fn default() -> Self {
                Schedule {
                        subject:        "N/A".to_string(),
                        room:           "".to_string(),
                        spotted:        false,
                        has_conflict:   false,

                        day:            "Su".to_string(),
                        start:          Time::from_hms(0,0,0).unwrap(),
                        end:            Time::from_hms(0,0,0).unwrap(),
                        duration:       Duration::default(),
                }
        }
}

/*
        Bound Schedules:
        Bound schedules are schedule entries that belong to either of the ff:
                1. Multi-day regular subjects.
                        E.g.
                        POC 7:30AM - 9:30AM [Mon, Fri]
                2. Multi-segment regular subjects.
                        E.g.
                        ITC 1:30PM - 4:00PM [Tue - Laboratory]
                        ITC 7:30AM - 9:00AM [Wed - Lecture]

        Bounds:
        Bounds serve as the criterion for a subject to be active inside a Week
                Bounds:
                1. Multi-day bounds:
                        E.g.
                        Monday, Tues, Wed
                2. Multi-segment bounds or Block-assigned bounds:
                        E.g.
                        Tuesday - Lab (Block IT101A)
                        Thursday - Lec (Block IT101A)
                3. Period-duration bounds:
                        E.g
                        Sub A: 3H   <--- X --->   Sub B: 1.3H
                        Sub A: 3H   <--- X --->   Sub C: 3H
*/
pub(crate) type Bound = Vec<Schedule>;
