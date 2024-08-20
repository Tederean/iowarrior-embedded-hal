use crate::backend::PipeInfo;
use crate::iowarrior::{iowarrior_service, HidError, IOWarrior, IOWarriorType};
use std::fmt;

#[derive(Debug, Clone)]
pub struct IOWarriorInfo {
    pipe_0: PipeInfo,
    pipe_1: PipeInfo,
    pipe_2: Option<PipeInfo>,
    pipe_3: Option<PipeInfo>,
    device_base_type: IOWarriorType,
    possible_device_types: Vec<IOWarriorType>,
    device_uuid: String,
    device_serial: Option<String>,
    device_revision: Option<u16>,
}

impl IOWarriorInfo {
    pub(crate) fn new(
        pipe_0: PipeInfo,
        pipe_1: PipeInfo,
        pipe_2: Option<PipeInfo>,
        pipe_3: Option<PipeInfo>,
        device_base_type: IOWarriorType,
        possible_device_types: Vec<IOWarriorType>,
        device_uuid: String,
        device_serial: Option<String>,
        device_revision: Option<u16>,
    ) -> IOWarriorInfo {
        IOWarriorInfo {
            pipe_0,
            pipe_1,
            pipe_2,
            pipe_3,
            device_serial,
            device_base_type,
            possible_device_types,
            device_uuid,
            device_revision,
        }
    }

    pub fn possible_device_types(&self) -> &[IOWarriorType] {
        self.possible_device_types.as_slice()
    }

    pub fn serial_number(&self) -> Option<&str> {
        self.device_serial.as_ref().map(|s| &**s)
    }

    pub fn revision(&self) -> Option<u16> {
        self.device_revision
    }

    pub fn open(&self) -> Result<IOWarrior, HidError> {
        iowarrior_service::open_iowarrior(
            self.pipe_0.clone(),
            self.pipe_1.clone(),
            self.pipe_2.clone(),
            self.pipe_3.clone(),
            self.device_uuid.clone(),
            self.device_serial.clone(),
            self.device_revision,
            self.device_base_type,
        )
    }
}

impl fmt::Display for IOWarriorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
