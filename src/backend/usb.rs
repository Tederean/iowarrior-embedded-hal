use crate::iowarrior::HidError;

#[derive(Clone)]
pub struct PipeInfo {
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        todo!()
    }

    pub fn pipe(&self) -> u8 {
        todo!()
    }

    pub fn serial_number(&self) -> Option<&str> {
        todo!()
    }

    pub fn product_id(&self) -> u16 {
        todo!()
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        todo!()
    }
}

pub struct PipeImpl {
}

impl PipeImpl {
    pub fn write_report(&mut self, _report: &[u8]) -> Result<usize, HidError> {
        todo!()
    }

    pub fn read_report_non_blocking(&mut self, _report: &mut [u8]) -> Result<usize, HidError> {
        todo!()
    }

    pub fn read_report(&mut self, _report: &mut [u8]) -> Result<usize, HidError> {
        todo!()
    }

    pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
        todo!()
    }
}
