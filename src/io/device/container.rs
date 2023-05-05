use crate::io::DeferredDevice;
use std::collections::HashMap;

/// Alias for using a deferred devices in `Container`, indexed by `K`
pub type DeviceContainer<K> = HashMap<K, DeferredDevice>;
