use crate::backend::VENDOR_ID;
use crate::iowarrior::HidError;
use rusb::DeviceDescriptor;
use std::sync::Arc;

#[derive(Clone)]
pub struct PipeInfo {
    device_descriptor: Arc<DeviceDescriptor>,
    interface: u8,
    device_uuid: String,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        let device_list = rusb::devices().map_err(|x| map_usb_error(x))?;

        let mut pipes = Vec::new();

        for device in device_list.iter() {
            let device_descriptor = match device.device_descriptor() {
                Ok(x) => Arc::new(x),
                Err(_) => continue,
            };

            match device_descriptor.vendor_id() {
                VENDOR_ID => {}
                _ => continue,
            };

            let config_descriptor = match device.active_config_descriptor() {
                Ok(x) => x,
                Err(_) => continue,
            };

            let device_uuid = format!("{0}-{1}", device.bus_number(), device.address());

            for interface in config_descriptor.interfaces() {
                pipes.push(PipeInfo {
                    device_descriptor: device_descriptor.clone(),
                    interface: interface.number(),
                    device_uuid: device_uuid.clone(),
                });
            }
        }

        Ok(pipes)
    }

    pub fn product_id(&self) -> u16 {
        self.device_descriptor.product_id()
    }

    pub fn pipe(&self) -> u8 {
        self.interface
    }

    pub fn uuid(&self) -> &str {
        self.device_uuid.as_ref()
    }

    pub fn metadata(&self) -> Result<(Option<String>, Option<u16>), HidError> {
        todo!()
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        todo!()
    }
}

pub struct PipeImpl {}

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
}

#[inline]
fn map_usb_error(usb_error: rusb::Error) -> HidError {
    HidError::HidApiError {
        message: usb_error.to_string(),
    }
}