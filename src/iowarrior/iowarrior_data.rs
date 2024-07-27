use crate::iowarrior::{IOWarriorType, PipeName, Report};
use std::fmt;

#[derive(Debug)]
pub struct IOWarriorData {
    pub device_revision: Option<u16>,
    pub device_serial: Option<String>,
    pub device_type: IOWarriorType,
    pub standard_report_size: usize,
    pub special_report_size: usize,
}

impl IOWarriorData {
    pub fn create_report(&self, pipe_name: PipeName) -> Report {
        Report {
            buffer: match pipe_name {
                PipeName::IOPins => {
                    vec![0u8; self.standard_report_size]
                }

                PipeName::SpecialMode | PipeName::I2CMode | PipeName::ADCMode => {
                    vec![0u8; self.special_report_size]
                }
            },
            pipe: pipe_name,
        }
    }
}

impl fmt::Display for IOWarriorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
