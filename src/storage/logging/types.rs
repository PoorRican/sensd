use crate::helpers::Def;
use crate::io::IOEvent;
use crate::storage::Log;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Hashmap type alias defines a type alias `LogType` for storing `IOEvent` by `DateTime<Utc>` keys.
pub type EventCollection = HashMap<DateTime<Utc>, IOEvent>;

/// Primary container for storing `Log` instances.
pub type LogContainer = Vec<Def<Log>>;
