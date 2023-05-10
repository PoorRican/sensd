use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::Def;
use crate::io::{Device, IdTraits};
use std::collections::hash_map::{Entry, Iter, Values, ValuesMut};
use std::collections::HashMap;
use std::sync::Arc;
use crate::settings::Settings;

/// Alias for using a deferred devices in `Container`, indexed by `K`
#[derive(Default)]
pub struct DeviceContainer<K: IdTraits, D: Device>(HashMap<K, Def<D>>);

impl<K, D> DeviceContainer<K, D>
where
    K: IdTraits,
    D: Device,
{
    pub fn values(&self) -> Values<K, Def<D>> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<K, Def<D>> {
        self.0.values_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, id: K, device: Def<D>) -> Result<Def<D>, ErrorType> {
        match self.0.entry(id) {
            Entry::Occupied(_) => Err(Error::new(
                ErrorKind::ContainerError,
                "Device entry already exists",
            )),
            Entry::Vacant(entry) => Ok(entry.insert(device).clone()),
        }
    }

    pub fn get(&self, k: &K) -> Option<&Def<D>> {
        self.0.get(k)
    }

    pub fn iter(&self) -> Iter<K, Def<D>> {
        self.0.iter()
    }

    /// Call [`Device::set_settings()`] on all stored device objects
    pub fn set_settings(&mut self, settings: Arc<Settings>) {
        for binding in self.values_mut() {
            let device = binding.try_lock().unwrap();
            device.set_settings(settings.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;
    use crate::io::{Device, DeviceContainer, Output, Input};
    use crate::settings::Settings;
    use crate::storage::Chronicle;

    #[test]
    fn insert_output() {
        const ITERATIONS: u32 = 15;
        let mut container = DeviceContainer::default();

        assert_eq!(0, container.len());

        for id in 0..ITERATIONS {
            let output = Output::new("", id, None).into_deferred();

            assert!(
                container.insert(id, output)
                    .is_ok()
            );
            assert_eq!(
                (id + 1) as usize,
                container.len()
            );
        }

        for id in 0..ITERATIONS {
            let output = Output::new("", id, None).into_deferred();

            assert!(
                container.insert(id, output)
                    .is_err()
            );
            assert_eq!(
                ITERATIONS as usize,
                container.len()
            );
        }
    }


    #[test]
    fn insert_input() {
        const ITERATIONS: u32 = 15;
        let mut container = DeviceContainer::default();

        assert_eq!(0, container.len());

        for id in 0..ITERATIONS {
            let input = Input::new("", id, None).into_deferred();

            assert!(
                container.insert(id, input)
                    .is_ok()
            );
            assert_eq!(
                (id + 1) as usize,
                container.len()
            );
        }

        for id in 0..ITERATIONS {
            let input = Input::new("", id, None).into_deferred();

            assert!(
                container.insert(id, input)
                    .is_err()
            );
            assert_eq!(
                ITERATIONS as usize,
                container.len()
            );
        }
    }

    #[test]
    /// Ensure that [`Device::set_settings()`] is called on each device
    fn set_settings() {
        let mut settings = Settings::default();
        settings.set_root("New Root");

        let input = Input::new("", 0, None)
            .init_log();
        assert!(
            input.log()
                .unwrap().try_lock().unwrap().deref()
                .settings().is_none());

        let mut container = DeviceContainer::default();
        container.insert(0, input.into_deferred()).unwrap();

        let input = container.get(&0).unwrap()
                .try_lock().unwrap();
        input.set_settings(Arc::new(settings));

        assert!(
            input
                .log()
                .unwrap().try_lock().unwrap().deref()
                .settings().is_some());
    }

}