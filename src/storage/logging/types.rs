use crate::helpers::Def;
use crate::io::IOEvent;
use crate::storage::Log;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Mapped collection for storing [`IOEvent`]s by [`DateTime<Utc>`] keys
///
/// All events should originate from a single source.
pub type EventCollection = HashMap<DateTime<Utc>, IOEvent>;

/// Primary container for storing multiple [`Log`] instances
///
/// [`Log`] instances may belong to a single source or multiple sources.
pub type LogContainer = Vec<Def<Log>>;
