use crate::iowarrior::{HidError, Peripheral, Pipe, PipeName};
use crate::iowarrior::{Report, UsedPin};
use std::fmt;

#[derive(Debug)]
pub struct IOWarriorMutData {
    pub pipe_0: Pipe,
    pub pipe_1: Pipe,
    pub pipe_2: Option<Pipe>,
    pub pipe_3: Option<Pipe>,
    pub pins_in_use: Vec<UsedPin>,
    pub dangling_peripherals: Vec<Peripheral>,
    pub pins_write_report: Report,
    pub pins_read_report: Report,
}

impl IOWarriorMutData {
    pub fn write_report(&mut self, report: &Report) -> Result<(), HidError> {
        let pipe = self.get_pipe(report.pipe);

        pipe.write_report(report)
    }

    pub fn read_report_non_blocking(&mut self, report: &mut Report) -> Result<bool, HidError> {
        let pipe = self.get_pipe(report.pipe);

        pipe.read_report_non_blocking(report)
    }

    pub fn read_report(&mut self, report: &mut Report) -> Result<(), HidError> {
        let pipe = self.get_pipe(report.pipe);

        pipe.read_report(report)
    }

    #[inline]
    fn get_pipe(&mut self, pipe_name: PipeName) -> &mut Pipe {
        match pipe_name {
            PipeName::IOPins => &mut self.pipe_0,
            PipeName::SpecialMode => &mut self.pipe_1,
            PipeName::I2CMode => self.pipe_2.as_mut().unwrap_or(&mut self.pipe_1),
            PipeName::ADCMode => self.pipe_3.as_mut().unwrap_or(&mut self.pipe_1),
        }
    }
}

impl fmt::Display for IOWarriorMutData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
