use crate::iowarrior::HidError;
use std::sync::Mutex;

static LOCK_STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[derive(Debug)]
pub struct IOWarriorLock {
    device_serial: String,
}

impl IOWarriorLock {
    pub fn new(device_serial: String) -> Result<IOWarriorLock, HidError> {
        let mut device_serials = LOCK_STORAGE.lock().unwrap();

        match device_serials.contains(&device_serial) {
            true => Err(HidError::HidApiError {
                message: "Device already opened.".to_string(),
            }),
            false => {
                device_serials.push(device_serial.clone());

                Ok(IOWarriorLock { device_serial })
            }
        }
    }
}

impl Drop for IOWarriorLock {
    fn drop(&mut self) {
        let mut device_serials = LOCK_STORAGE.lock().unwrap();

        device_serials.retain(|x| !x.eq(&self.device_serial))
    }
}
