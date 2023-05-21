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

#[cfg(test)]
mod test_event_collection {
    use chrono::{Duration, Utc};
    use crate::io::{IOEvent, RawValue};
    use crate::storage::EventCollection;

    fn generate_log(count: usize) -> EventCollection {
        let mut log = EventCollection::default();

        let now = Utc::now();

        for i in 0..count {
            let timestamp = now - Duration::seconds(i as i64);
            let event = IOEvent::with_timestamp(timestamp, RawValue::Binary(true));

            log.insert(timestamp, event);
        }

        log
    }

    #[test]
    fn test_extendability() {
        let mut orig = generate_log(5);
        let ext = generate_log(5);

        assert_eq!(5, orig.len());
        orig.extend(ext);
        assert_eq!(10, orig.len());
    }
}