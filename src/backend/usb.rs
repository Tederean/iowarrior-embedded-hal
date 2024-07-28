use crate::backend::{VENDOR_ID};
use crate::iowarrior::HidError;
use rusb::{Device, DeviceDescriptor, DeviceHandle, Error, GlobalContext};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct PipeInfo {
    device_descriptor: Arc<DeviceDescriptor>,
    device: Arc<Device<GlobalContext>>,
    interface: u8,
    device_uuid: String,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        let device_list = rusb::devices().map_err(|x| map_usb_error(x))?;

        let mut pipes = Vec::new();

        for device in device_list.iter() {
            let device_descriptor = match device.device_descriptor() {
                Ok(x) => x,
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
            let device_arc = Arc::new(device);
            let device_descriptor_arc = Arc::new(device_descriptor);

            for interface in config_descriptor.interfaces() {
                pipes.push(PipeInfo {
                    device_descriptor: device_descriptor_arc.clone(),
                    interface: interface.number(),
                    device_uuid: device_uuid.clone(),
                    device: device_arc.clone(),
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
        let device_handle = self.device.open().map_err(|x| map_usb_error(x))?;

        let device_serial = self.serial_number(&device_handle)?;
        let device_revision = self.revision(&device_handle)?;

        Ok((device_serial, device_revision))
    }

    fn serial_number(&self, device_handle: &DeviceHandle<GlobalContext>) -> Result<Option<String>, HidError> {
        let timeout = Duration::from_millis(100);

        let languages = device_handle.read_languages(timeout).map_err(|x| map_usb_error(x))?;

        let language = match languages.get(0) {
            None => return Ok(None),
            Some(x) => x.clone(),
        };

        let device_serial = match device_handle.read_serial_number_string(language, &self.device_descriptor, timeout) {
            Ok(x) => Some(x),
            Err(x) => {
                match x {
                    Error::InvalidParam => None,
                    _ => return Err(map_usb_error(x)),
                }
            }
        };

        Ok(device_serial)
    }

    fn revision(&self, _device_handle: &DeviceHandle<GlobalContext>) -> Result<Option<u16>, HidError> {
        todo!();

        //let path =

        //get_revision(path)
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
fn map_usb_error(usb_error: Error) -> HidError {
    HidError::HidApiError {
        message: usb_error.to_string(),
    }
}