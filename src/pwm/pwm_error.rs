use crate::iowarrior::HidError;
use embedded_hal::pwm::ErrorKind;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum PWMError {
    #[error("USB HID error.")]
    ErrorUSB(HidError),
}

impl embedded_hal::pwm::Error for PWMError {
    fn kind(&self) -> ErrorKind {
        match self {
            PWMError::ErrorUSB(_) => ErrorKind::Other,
        }
    }
}
