use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PipeName {
    IOPins = 0,
    SpecialMode = 1,
    I2CMode = 2,
    ADCMode = 3,
}

impl fmt::Display for PipeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PipeName {
    #[inline]
    #[allow(dead_code)]
    pub fn get_value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    #[allow(dead_code)]
    pub fn from_value(value: u8) -> Option<PipeName> {
        match value {
            0 => Some(PipeName::IOPins),
            1 => Some(PipeName::SpecialMode),
            2 => Some(PipeName::I2CMode),
            3 => Some(PipeName::ADCMode),
            _ => None,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_special_report(&self) -> bool {
        match self {
            PipeName::IOPins => false,
            PipeName::SpecialMode | PipeName::I2CMode | PipeName::ADCMode => true,
        }
    }
}
