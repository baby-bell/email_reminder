use std::cmp::Ordering;

use chrono::DateTime;
use chrono::offset::local::Local;

type Time = DateTime<Local>;

#[derive(Debug, PartialEq, Eq)]
struct Event {
    name: String,
    location: Option<String>,
    time: Time,
    reminders: Vec<Time>,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time == other.time {
            if self.location == other.location {
                return self.name.cmp(&other.name);
            }
            return self.location.cmp(&other.location);
        }
        return self.time.cmp(&other.time);
    }
}

pub struct EventTable {
    events: BTreeSet<Event>,
}