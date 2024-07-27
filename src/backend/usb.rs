use std::sync::Arc;
use std::time::Duration;
use rusb::{DeviceDescriptor, DeviceHandle, GlobalContext};
use crate::iowarrior::HidError;
use crate::backend::VENDOR_ID;

static USB_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct PipeInfo {
    device_handle: Arc<DeviceHandle<GlobalContext>>,
    device_descriptor: Arc<DeviceDescriptor>,
    device_serial: Option<String>,
    interface: u8,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        collect().map_err(|x| map_usb_error(x))
    }

    pub fn pipe(&self) -> u8 {
        self.interface
    }

    pub fn serial_number(&self) -> Option<&str> {
        self.device_serial.as_ref().map(|s| &**s)
    }

    pub fn product_id(&self) -> u16 {
        self.device_descriptor.product_id()
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        todo!()
    }
}

fn collect() -> Result<Vec<PipeInfo>, rusb::Error> {
    let device_list = rusb::devices()?;

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

        let device_handle = Arc::new(device.open()?);

        let languages = device_handle.read_languages(USB_TIMEOUT)?;

        let language = languages.get(0).unwrap().clone();

        let device_serial = device_handle.read_serial_number_string(language, &device_descriptor, USB_TIMEOUT).ok();

        let config_descriptor = device.active_config_descriptor()?;

        for interface in config_descriptor.interfaces() {

            pipes.push(PipeInfo {
                device_handle: device_handle.clone(),
                device_descriptor: device_descriptor.clone(),
                device_serial: device_serial.clone(),
                interface: interface.number(),
            })
        }
    }

    Ok(pipes)
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

    pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
        todo!()
    }
}

#[inline]
fn map_usb_error(usb_error: rusb::Error) -> HidError {
    HidError::HidApiError { message: usb_error.to_string() }
}