use crate::backend::PipeInfo;
use crate::iowarrior::{iowarrior_service, HidError, IOWarrior, IOWarriorType};
use std::fmt;

#[derive(Debug, Clone)]
pub struct IOWarriorInfo {
    pipe_0: PipeInfo,
    pipe_1: PipeInfo,
    pipe_2: Option<PipeInfo>,
    pipe_3: Option<PipeInfo>,
    device_serial: String,
    device_base_type: IOWarriorType,
    possible_device_types: Vec<IOWarriorType>,
}

impl IOWarriorInfo {
    pub(crate) fn new(
        pipe_0: PipeInfo,
        pipe_1: PipeInfo,
        pipe_2: Option<PipeInfo>,
        pipe_3: Option<PipeInfo>,
        device_serial: String,
        device_base_type: IOWarriorType,
        possible_device_types: Vec<IOWarriorType>,
    ) -> IOWarriorInfo {
        IOWarriorInfo {
            pipe_0,
            pipe_1,
            pipe_2,
            pipe_3,
            device_serial,
            device_base_type,
            possible_device_types,
        }
    }

    pub fn get_possible_device_types(&self) -> &[IOWarriorType] {
        self.possible_device_types.as_slice()
    }

    pub fn get_serial_number(&self) -> &str {
        &self.device_serial
    }

    pub fn open(&self) -> Result<IOWarrior, HidError> {
        iowarrior_service::open_iowarrior(
            self.pipe_0.clone(),
            self.pipe_1.clone(),
            self.pipe_2.clone(),
            self.pipe_3.clone(),
            self.device_serial.clone(),
            self.device_base_type,
        )
    }
}

impl fmt::Display for IOWarriorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
