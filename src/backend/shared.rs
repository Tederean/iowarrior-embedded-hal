use crate::backend::{PipeImpl, PipeInfo};
use crate::iowarrior::HidError;
use std::fmt;
use std::fmt::Debug;
use std::path::Path;

#[allow(dead_code)]
pub const VENDOR_ID: u16 = 0x07c0;

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

impl Debug for PipeImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PipeImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {

        #[allow(dead_code)]
        pub fn get_revision<P: AsRef<Path>>(path: &P) -> Result<Option<u16>, HidError> {
            use std::fs::File;
            use std::os::windows::io::AsRawHandle;
            use windows::Win32::Devices::HumanInterfaceDevice::{HidD_GetAttributes, HIDD_ATTRIBUTES};
            use windows::Win32::Foundation::{BOOLEAN, HWND};

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

        #[allow(dead_code)]
        pub fn get_revision<P: AsRef<Path>>(path: &P) -> Result<Option<u16>, HidError> {
            use std::fs::OpenOptions;
            use std::os::fd::AsRawFd;
            use std::os::raw;

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

            let file = OpenOptions::new()
                .read(true)
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

        #[allow(dead_code)]
        pub fn get_revision<P: AsRef<Path>>(_path: &P) -> Result<Option<u16>, HidError> {
            Ok(None)
        }

    }
}
