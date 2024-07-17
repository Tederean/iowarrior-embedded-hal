use crate::bits::Bit;
use crate::bits::Bitmasking;
use crate::digital::{InputPin, OutputPin, PinError, PinSetupError};
use crate::iowarrior::{
    peripheral_service, IOWarriorData, IOWarriorMutData, IOWarriorType, PipeName, UsedPin,
};
use embedded_hal::digital::PinState;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn new_input(
    data: &Arc<IOWarriorData>,
    mut_data_mutex: &Arc<Mutex<IOWarriorMutData>>,
    pin: u8,
) -> Result<InputPin, PinSetupError> {
    let mut mut_data = mut_data_mutex.lock().unwrap();

    enable_gpio(&data, &mut mut_data, PinState::High, pin)?;

    Ok(InputPin {
        pin,
        data: data.clone(),
        mut_data_mutex: mut_data_mutex.clone(),
    })
}

pub fn new_output(
    data: &Arc<IOWarriorData>,
    mut_data_mutex: &Arc<Mutex<IOWarriorMutData>>,
    pin_state: PinState,
    pin: u8,
) -> Result<OutputPin, PinSetupError> {
    let mut mut_data = mut_data_mutex.lock().unwrap();

    enable_gpio(&data, &mut mut_data, pin_state, pin)?;

    Ok(OutputPin {
        pin,
        mut_data_mutex: mut_data_mutex.clone(),
    })
}

fn enable_gpio(
    data: &IOWarriorData,
    mut_data: &mut MutexGuard<IOWarriorMutData>,
    pin_state: PinState,
    pin: u8,
) -> Result<(), PinSetupError> {
    if data.device_type == IOWarriorType::IOWarrior28Dongle
        || data.device_type == IOWarriorType::IOWarrior56Dongle
    {
        return Err(PinSetupError::NotSupported);
    }

    if !get_is_valid_gpio(data.device_type, pin) {
        return Err(PinSetupError::PinNotExisting);
    }

    match mut_data.pins_in_use.iter().filter(|x| x.pin == pin).next() {
        None => {}
        Some(used_pin) => {
            return Err(match used_pin.peripheral {
                None => PinSetupError::AlreadySetup,
                Some(peripheral) => PinSetupError::BlockedByPeripheral(peripheral),
            })
        }
    }

    peripheral_service::cleanup_dangling_modules(&data, mut_data)
        .map_err(|x| PinSetupError::ErrorUSB(x))?;

    peripheral_service::set_pin_output(mut_data, pin_state, pin)
        .map_err(|x| PinSetupError::ErrorUSB(x))?;

    mut_data.pins_in_use.push(UsedPin {
        pin,
        peripheral: None,
    });

    Ok(())
}

pub fn is_pin_input_state(
    data: &Arc<IOWarriorData>,
    mut_data: &mut MutexGuard<IOWarriorMutData>,
    pin: u8,
    expected_pin_state: PinState,
) -> Result<bool, PinError> {
    let mut report = data.create_report(PipeName::IOPins);

    if mut_data
        .read_report_non_blocking(&mut report)
        .map_err(|x| PinError::ErrorUSB(x))?
    {
        mut_data.pins_read_report = report;
    }

    let byte_index = ((pin as usize) / 8usize) + 1;
    let bit_index = Bit::from_u8(pin % 8u8);

    let value = mut_data.pins_read_report.buffer[byte_index].get_bit(bit_index);

    Ok(match expected_pin_state {
        PinState::Low => !value,
        PinState::High => value,
    })
}

pub fn set_pin_output_state(
    mut_data: &mut MutexGuard<IOWarriorMutData>,
    pin: u8,
    pin_state: PinState,
) -> Result<(), PinError> {
    peripheral_service::set_pin_output(mut_data, pin_state, pin).map_err(|x| PinError::ErrorUSB(x))
}

pub fn is_pin_output_state(
    mut_data: &mut MutexGuard<IOWarriorMutData>,
    pin: u8,
    expected_pin_state: PinState,
) -> Result<bool, PinError> {
    let byte_index = ((pin as usize) / 8usize) + 1;
    let bit_index = Bit::from_u8(pin % 8u8);

    let value = mut_data.pins_write_report.buffer[byte_index].get_bit(bit_index);

    Ok(match expected_pin_state {
        PinState::Low => !value,
        PinState::High => value,
    })
}

fn get_is_valid_gpio(device_type: IOWarriorType, pin: u8) -> bool {
    match device_type {
        IOWarriorType::IOWarrior40 => pin < 32,
        IOWarriorType::IOWarrior24 => pin < 16,
        IOWarriorType::IOWarrior24PowerVampire => pin < 12,
        IOWarriorType::IOWarrior28 => pin < 18 || pin == 31,
        IOWarriorType::IOWarrior28Dongle | IOWarriorType::IOWarrior56Dongle => false,
        IOWarriorType::IOWarrior28L => pin < 18,
        IOWarriorType::IOWarrior56 => pin < 49 || pin == 55,
        IOWarriorType::IOWarrior100 => {
            pin < 11 || (pin > 15 && pin < 84) || pin == 86 || pin == 89 || pin == 90
        }
    }
}
