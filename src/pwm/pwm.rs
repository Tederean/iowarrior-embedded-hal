use crate::iowarrior::Peripheral;
use crate::iowarrior::{peripheral_service, IOWarriorData, IOWarriorMutData};
use crate::pwm::{pwm_service, PWMChannel, PWMConfig, PWMData, PWMError};
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

#[derive(Debug)]
pub struct PWM {
    pub(crate) data: Arc<IOWarriorData>,
    pub(crate) mut_data_mutex: Arc<Mutex<IOWarriorMutData>>,
    pub(crate) pwm_data_rwlock: Arc<RwLock<PWMData>>,
    pub(crate) channel: PWMChannel,
}

impl Drop for PWM {
    fn drop(&mut self) {
        let mut pwm_data = self.pwm_data_rwlock.write().unwrap();

        pwm_data.pins_counter -= 1;

        if pwm_data.pins_counter == 0 {
            peripheral_service::disable_peripheral(
                &self.data,
                &mut self.mut_data_mutex.lock().unwrap(),
                Peripheral::PWM,
            );
        }
    }
}

impl fmt::Display for PWM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl embedded_hal::pwm::ErrorType for PWM {
    type Error = PWMError;
}

impl embedded_hal::pwm::SetDutyCycle for PWM {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        self.pwm_data_rwlock.read().unwrap().max_duty_cycle
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let mut mut_data = self.mut_data_mutex.lock().unwrap();
        let mut pwm_data = self.pwm_data_rwlock.write().unwrap();

        pwm_data.set_duty_cycle(self.channel, duty);

        pwm_service::update_duty_cycle(&self.data, &mut mut_data, &pwm_data)
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::PwmPin for PWM {
    type Duty = u16;

    #[inline]
    fn disable(&mut self) {}

    #[inline]
    fn enable(&mut self) {}

    #[inline]
    fn get_duty(&self) -> Self::Duty {
        self.pwm_data_rwlock.read().unwrap().get_duty_cycle(self.channel)
    }

    #[inline]
    fn get_max_duty(&self) -> Self::Duty {
        self.pwm_data_rwlock.read().unwrap().max_duty_cycle
    }

    #[inline]
    fn set_duty(&mut self, duty: Self::Duty) {
        let mut mut_data = self.mut_data_mutex.lock().unwrap();
        let mut pwm_data = self.pwm_data_rwlock.write().unwrap();

        pwm_data.set_duty_cycle(self.channel, duty);

        _ = pwm_service::update_duty_cycle(&self.data, &mut mut_data, &pwm_data)
    }
}

impl PWM {
    #[inline]
    pub fn get_config(&self) -> PWMConfig {
        self.pwm_data_rwlock.read().unwrap().pwm_config.clone()
    }

    #[inline]
    pub fn get_frequency_hz(&self) -> u32 {
        self.pwm_data_rwlock.read().unwrap().calculated_frequency_hz
    }

    #[inline]
    pub fn get_channel(&self) -> PWMChannel {
        self.channel
    }

    #[inline]
    pub fn get_duty_cycle(&self) -> u16 {
        self.pwm_data_rwlock.read().unwrap().get_duty_cycle(self.channel)
    }

    #[inline]
    pub fn get_max_duty_cycle(&self) -> u16 {
        self.pwm_data_rwlock.read().unwrap().max_duty_cycle
    }
}
