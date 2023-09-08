use time::{Time, Duration};

#[derive(Clone, PartialOrd, PartialEq)]
pub(crate) enum Schedule {
        RegularV(Regular),
        BoundV(Bound)
}

#[derive(Default, Clone, PartialOrd, PartialEq)]
pub(crate) struct Regular {
        // Subject Metadata
        pub(crate) base_d:              BaseData,

        // Adaptive
        pub(crate) session_d:           SessionData,
        pub(crate) time_d:              TimeData,
}

#[derive(Default, Clone, PartialOrd, PartialEq)]
pub(crate) struct Bound {
        // Subject Metadata
        pub(crate) base_d:              BaseData,

        // Time Metadata
        pub(crate) session_d:           Vec<SessionData>,
        pub(crate) time_d:              Vec<TimeData>,
}


// ---------- Sub Components ----------

#[derive(Default, Clone, PartialOrd, PartialEq)]
pub(crate) struct BaseData {
        // Base subject metadata
        pub(crate) subject:             String,
        pub(crate) spotted:             bool,
        pub(crate) has_conflict:        bool,
        pub(crate) priority:            u32,
}

#[derive(Default, Clone, PartialOrd, PartialEq)]
pub(crate) struct SessionData {
        // Session specific metadata
        pub(crate) room:                String,
        pub(crate) block:               String,
        pub(crate) mode:                String,
}

#[derive(Clone, PartialOrd, PartialEq)]
pub(crate) struct TimeData {
        // Session time schedule metadata
        pub(crate) day:                 String,
        pub(crate) start:               Time,
        pub(crate) end:                 Time,
        pub(crate) duration:            Duration,
}

impl Default for TimeData {
        fn default() -> Self {
                TimeData {
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
