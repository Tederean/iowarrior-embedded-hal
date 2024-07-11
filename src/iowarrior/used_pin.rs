use crate::iowarrior::Peripheral;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UsedPin {
    pub pin: u8,
    pub peripheral: Option<Peripheral>,
}

impl fmt::Display for UsedPin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
