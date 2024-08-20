use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IOWarriorType {
    IOWarrior40,
    IOWarrior24,
    IOWarrior24Dongle,
    IOWarrior24PowerVampire,
    IOWarrior56,
    IOWarrior56Dongle,
    IOWarrior28,
    IOWarrior28Dongle,
    IOWarrior28L,
    IOWarrior100,
}

impl IOWarriorType {
    #[inline]
    pub(crate) fn from_device_product_id(device_product_id: u16) -> Option<IOWarriorType> {
        match device_product_id {
            5376 => Some(IOWarriorType::IOWarrior40),
            5377 => Some(IOWarriorType::IOWarrior24),
            5393 | 5394 => Some(IOWarriorType::IOWarrior24PowerVampire),
            5379 => Some(IOWarriorType::IOWarrior56),
            5380 => Some(IOWarriorType::IOWarrior28),
            5381 => Some(IOWarriorType::IOWarrior28L),
            5382 => Some(IOWarriorType::IOWarrior100),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pipe_count(&self) -> u8 {
        match self {
            IOWarriorType::IOWarrior24
            | IOWarriorType::IOWarrior24Dongle
            | IOWarriorType::IOWarrior40
            | IOWarriorType::IOWarrior24PowerVampire
            | IOWarriorType::IOWarrior28L
            | IOWarriorType::IOWarrior56
            | IOWarriorType::IOWarrior56Dongle => 2,
            IOWarriorType::IOWarrior28
            | IOWarriorType::IOWarrior28Dongle
            | IOWarriorType::IOWarrior100 => 4,
        }
    }
}

impl fmt::Display for IOWarriorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
