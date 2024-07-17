use crate::iowarrior::HidError;
use hidapi::{DeviceInfo, HidApi, HidDevice};
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

const VENDOR_ID: u16 = 1984;

#[derive(Clone)]
pub struct PipeInfo {
    api: Rc<HidApi>,
    device_info: DeviceInfo,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        let api = Rc::new(HidApi::new().map_err(|x| map_hid_error(x))?);

        Ok(api
            .device_list()
            .filter_map(|x| match x.vendor_id() == VENDOR_ID {
                true => Some(PipeInfo {
                    api: api.clone(),
                    device_info: x.clone(),
                }),
                false => None,
            })
            .collect())
    }

    pub fn pipe(&self) -> u8 {
        self.device_info.interface_number() as u8
    }

    pub fn serial_number(&self) -> Option<&str> {
        self.device_info.serial_number()
    }

    pub fn product_id(&self) -> u16 {
        self.device_info.product_id()
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        let hid_device = self
            .api
            .open_path(self.device_info.path())
            .map_err(|x| map_hid_error(x))?;

        Ok(PipeImpl {
            hid_device,
            device_info: self.device_info,
        })
    }
}

impl Debug for PipeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PipeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct PipeImpl {
    hid_device: HidDevice,
    device_info: DeviceInfo,
}

impl PipeImpl {
    pub fn write_report(&mut self, report: &[u8]) -> Result<usize, HidError> {
        self.hid_device.write(report).map_err(|x| map_hid_error(x))
    }

    pub fn read_report_non_blocking(&mut self, report: &mut [u8]) -> Result<usize, HidError> {
        self.hid_device
            .set_blocking_mode(false)
            .map_err(|x| map_hid_error(x))?;

        self.hid_device.read(report).map_err(|x| map_hid_error(x))
    }

    pub fn read_report(&mut self, report: &mut [u8]) -> Result<usize, HidError> {
        self.hid_device
            .set_blocking_mode(true)
            .map_err(|x| map_hid_error(x))?;

        self.hid_device.read(report).map_err(|x| map_hid_error(x))
    }

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {

            pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
                use std::fs::File;
                use std::os::windows::io::AsRawHandle;
                use windows::Win32::Devices::HumanInterfaceDevice::{HidD_GetAttributes, HIDD_ATTRIBUTES};
                use windows::Win32::Foundation::{BOOLEAN, HWND};

                let path = match self.device_info.path().to_str() {
                    Ok(x) => x,
                    Err(_) => return Err(HidError::InitializationError),
                };

                let file = File::open(path).map_err(|x| HidError::IoError { error: x })?;

                let hwnd = HWND(file.as_raw_handle());

                let mut attributes = HIDD_ATTRIBUTES {
                    Size: std::mem::size_of::<HIDD_ATTRIBUTES>() as u32,
                    VendorID: 0,
                    ProductID: 0,
                    VersionNumber: 0,
                };

                match unsafe { HidD_GetAttributes(hwnd, &mut attributes) != BOOLEAN(0) } {
                    true => Ok(Some(attributes.VersionNumber)),
                    false => Err(HidError::InitializationError),
                }
            }

        } else if #[cfg(target_os = "linux")] {

            pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
                use std::fs::OpenOptions;
                use std::os::fd::AsRawFd;
                use std::os::raw;
                use std::str::Utf8Error;

                #[repr(C)]
                #[derive(Debug)]
                struct IoctlInfo {
                    vendor: raw::c_int,
                    product: raw::c_int,
                    serial: [raw::c_char; 9],
                    revision: raw::c_int,
                    speed: raw::c_int,
                    power: raw::c_int,
                    interface: raw::c_int,
                    packet_size: raw::c_uint,
                }

                nix::ioctl_read!(ioctl_info_iowarrior, 0xC0, 3, IoctlInfo);

                let path = match self.device_info.path().to_str() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(HidError::OpenHidDeviceWithDeviceInfoError {
                            path: self.device_info.path().into_c_string(),
                            vendor_id: self.device_info.vendor_id(),
                            product_id: self.device_info.product_id(),
                            interface_number: self.device_info.interface_number() as u8,
                        })
                    }
                };

                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(path)
                    .map_err(|x| HidError::IoError { error: x })?;

                let raw_file_descriptor = file.as_raw_fd();

                let mut ioctl_info = IoctlInfo {
                    vendor: 0,
                    product: 0,
                    serial: [0; 9],
                    revision: 0,
                    speed: 0,
                    power: 0,
                    interface: 0,
                    packet_size: 0,
                };

                match unsafe { ioctl_info_iowarrior(raw_file_descriptor, &mut ioctl_info) } {
                    Ok(_) => Ok(Some(ioctl_info.revision as u16)),
                    Err(_) => return Err(HidError::InitializationError),
                }
            }

        } else {

            pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
                Ok(None)
            }

        }
    }
}

impl fmt::Display for PipeImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[inline]
fn map_hid_error(hid_error: hidapi::HidError) -> HidError {
    match hid_error {
        hidapi::HidError::HidApiError { message } => HidError::HidApiError { message },
        hidapi::HidError::HidApiErrorEmpty => HidError::HidApiErrorEmpty,
        hidapi::HidError::FromWideCharError { .. } => HidError::HidApiErrorEmpty,
        hidapi::HidError::InitializationError => HidError::InitializationError,
        hidapi::HidError::InvalidZeroSizeData => HidError::InvalidZeroSizeData,
        hidapi::HidError::IncompleteSendError { sent, all } => {
            HidError::IncompleteSendError { sent, all }
        }
        hidapi::HidError::SetBlockingModeError { mode } => HidError::SetBlockingModeError { mode },
        hidapi::HidError::OpenHidDeviceWithDeviceInfoError { device_info } => {
            HidError::OpenHidDeviceWithDeviceInfoError {
                path: CString::from(device_info.path()),
                vendor_id: device_info.vendor_id(),
                product_id: device_info.product_id(),
                interface_number: device_info.interface_number() as u8,
            }
        }
        hidapi::HidError::IoError { error } => HidError::IoError { error },
    }
}
