use crate::io::DeferredDevice;
use crate::storage::Container;

/// Alias for using a deferred devices in `Container`, indexed by `K`
pub type DeviceContainer<K> = Container<DeferredDevice, K>;
