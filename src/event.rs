use std;
use std::cmp::Ordering;
use std::sync::Arc;
use std::collections::{BTreeSet, HashMap, Bound};
use std::collections::btree_set::{Iter, Range};
use std::iter::Filter;

use chrono::DateTime;
use chrono::offset::local::Local;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

type Time = DateTime<Local>;

/// An event that happens on a particular date and time.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Event {
    time: Time,
    name: Arc<String>,
    location: Option<Arc<String>>,
}

impl Event {
    /// Construct an event with the corresponding data.
    pub fn new(name: String, location: Option<String>, time: Time) -> Self {
        Event {
            name: Arc::new(name),
            location: location.map(Arc::new),
            time: time,
        }
    }
}

impl Serialize for Event {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Event", 3)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("name", &*self.name)?;
        if let Some(ref val) = self.location {
            state.serialize_field("location", val.as_ref())?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Time,
            Name,
            Location,
        };
        struct EventVisitor;
        impl<'de> Visitor<'de> for EventVisitor {
            type Value = Event;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Event")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Event, V::Error>
                where V: MapAccess<'de> {
                let (mut time, mut name, mut location) = (None, None, None);
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Time => {
                            if time.is_some() {
                                return Err(de::Error::duplicate_field("time"));
                            }
                            time = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Location => {
                            if location.is_some() {
                                return Err(de::Error::duplicate_field("location"));
                            }
                            location = Some(map.next_value()?);
                        }
                    }
                }
                let time = time.ok_or_else(|| de::Error::missing_field("time"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                Ok(Event::new(name, location, time))
            }
        }
        const FIELDS: &[&str] = &["time", "name", "location"];
        deserializer.deserialize_struct("Event", FIELDS, EventVisitor)
    }
}

/// A table of events and event reminders.
pub struct EventTable {
    events: BTreeSet<Event>,
    reminders: HashMap<Event, Vec<Time>>,
}

pub struct EventIter<'a> {
    inner: Range<'a, Event>,
}

impl<'a> Iterator for EventIter<'a> {
    type Item = &'a Event;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl EventTable {
    /// Create an `EventTable`.
    pub fn new() -> Self {
        EventTable {
            events: BTreeSet::new(),
            reminders: HashMap::new(),
        }
    }

    /// Add `e` to the table with no reminders.
    pub fn add_event(&mut self, e: Event) {
        self.events.insert(e.clone());
        self.reminders.insert(e, vec![]);
    }

    pub fn events_by_name(&self, name: &str) -> Vec<Event> {
        let v: Vec<_> = self.events.iter()
                                   .filter(|&x| &*x.name == name)
                                   .cloned()
                                   .collect();
        v
    }

    /// Get all events that lie within `[start, end)`.
    pub fn events_in_date_range<'a>(&'a self, start: Time, end: Time) -> EventIter<'a> {
        let range_start = Bound::Included(Event::new("".into(), None, start));
        let range_end = Bound::Excluded(Event::new("".into(), None, end));

        EventIter {
            inner: self.events.range((range_start, range_end)),
        }
    }

    /// Get the reminders for an event
    pub fn get_reminders<'a>(&'a self, e: &Event) -> Option<&'a Vec<Time>> {
        self.reminders.get(e)
    }
}