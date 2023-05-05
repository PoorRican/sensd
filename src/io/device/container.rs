use std::collections::HashMap;
use crate::io::DeferredDevice;

/// Alias for using a deferred devices in `Container`, indexed by `K`
pub type DeviceContainer<K> = HashMap<K, DeferredDevice>;
