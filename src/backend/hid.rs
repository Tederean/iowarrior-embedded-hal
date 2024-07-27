use crate::backend::{get_revision, VENDOR_ID};
use crate::iowarrior::HidError;
use hidapi::{DeviceInfo, HidApi, HidDevice};
use std::ffi::CString;
use std::sync::Arc;

#[derive(Clone)]
pub struct PipeInfo {
    api: Arc<HidApi>,
    device_info: DeviceInfo,
    device_uuid: String,
    device_path: String,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        let api = Arc::new({
            let mut api = HidApi::new_without_enumerate().map_err(|x| map_hid_error(x))?;

            api.add_devices(VENDOR_ID, 0)
                .map_err(|x| map_hid_error(x))?;

            api
        });

        Ok(api
            .device_list()
            .filter_map(|device_info| {
                let device_path = match device_info.path().to_str() {
                    Ok(x) => x.to_string(),
                    Err(_) => return None,
                };

                let device_uuid = get_device_uuid(&device_path);

                Some(PipeInfo {
                    api: api.clone(),
                    device_info: device_info.clone(),
                    device_uuid,
                    device_path,
                })
            })
            .collect())
    }

    pub fn product_id(&self) -> u16 {
        self.device_info.product_id()
    }

    pub fn pipe(&self) -> u8 {
        self.device_info.interface_number() as u8
    }

    pub fn uuid(&self) -> &str {
        self.device_uuid.as_ref()
    }

    pub fn metadata(&self) -> Result<(Option<String>, Option<u16>), HidError> {
        let device_revision = get_revision(&self.device_path)?;

        Ok((
            self.device_info.serial_number().map(|x| x.to_string()),
            device_revision,
        ))
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        let hid_device = self
            .api
            .open_path(self.device_info.path())
            .map_err(|x| map_hid_error(x))?;

        Ok(PipeImpl { hid_device })
    }
}

pub struct PipeImpl {
    hid_device: HidDevice,
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
}

#[inline]
fn get_device_uuid(path: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            String::from(&path[path.len() - 37..path.len() - 1])
        }
        else {
            compile_error!("TODO: Implement on macOS");
        }
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
