use crate::io::{
    DeviceType, DeviceWrapper, IOKind, IdType, IODirection, DeviceTraits
};
use crate::helpers::Def;

pub type DeferredDevice = Def<DeviceType>;
impl DeviceWrapper for DeferredDevice {
    fn is_input(&self) -> bool {
        let binding = self.try_lock().unwrap();
        binding.is_input()
    }
    fn is_output(&self) -> bool {
        let binding = self.try_lock().unwrap();
        binding.is_output()
    }
}
impl DeviceTraits for DeferredDevice {
    fn name(&self) -> String {
        self.try_lock().unwrap().name()
    }

    fn id(&self) -> IdType {
        self.try_lock().unwrap().id()
    }

    fn kind(&self) -> IOKind {
        self.try_lock().unwrap().kind()
    }

    fn direction(&self) -> IODirection {
        self.try_lock().unwrap().direction()
    }
}

