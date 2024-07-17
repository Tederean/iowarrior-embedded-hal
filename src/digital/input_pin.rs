use crate::digital::digital_service;
use crate::digital::PinError;
use crate::iowarrior::{peripheral_service, IOWarriorData, IOWarriorMutData};
use embedded_hal::digital::PinState;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct InputPin {
    pub(crate) data: Arc<IOWarriorData>,
    pub(crate) mut_data_mutex: Arc<Mutex<IOWarriorMutData>>,
    pub(crate) pin: u8,
}

impl embedded_hal::digital::ErrorType for InputPin {
    type Error = PinError;
}

impl embedded_hal::digital::InputPin for InputPin {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        digital_service::is_pin_input_state(
            &self.data,
            &mut self.mut_data_mutex.lock().unwrap(),
            self.pin,
            PinState::High,
        )
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        digital_service::is_pin_input_state(
            &self.data,
            &mut self.mut_data_mutex.lock().unwrap(),
            self.pin,
            PinState::Low,
        )
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::digital::v2::InputPin for InputPin {
    type Error = PinError;

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        digital_service::is_pin_input_state(
            &self.data,
            &mut self.mut_data_mutex.lock().unwrap(),
            self.pin,
            PinState::High,
        )
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        digital_service::is_pin_input_state(
            &self.data,
            &mut self.mut_data_mutex.lock().unwrap(),
            self.pin,
            PinState::Low,
        )
    }
}

impl fmt::Display for InputPin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Drop for InputPin {
    #[inline]
    fn drop(&mut self) {
        peripheral_service::disable_gpio(&mut self.mut_data_mutex.lock().unwrap(), self.pin);
    }
}
