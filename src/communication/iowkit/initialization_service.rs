use crate::communication::IowkitData;
use crate::communication::{CommunicationData, InitializationError};
use crate::iowarrior::{iowarrior_service, IOWarrior, IOWarriorType, Pipe};
use std::ptr::NonNull;
use std::sync::Arc;

#[cfg(target_os = "windows")]
const IOWKIT: &str = "iowkit.dll";

#[cfg(target_os = "linux")]
const IOWKIT: &str = "libiowkit.so";

pub fn get_iowarriors() -> Result<Vec<IOWarrior>, InitializationError> {
    let iowkit = unsafe { iowkit_sys::Iowkit::new(IOWKIT) }.map_err(|x| {
        InitializationError::InternalError("Error loading iowkit library.".to_owned())
    })?;

    let iowkit_handle = match NonNull::new(unsafe { iowkit.IowKitOpenDevice() }) {
        None => return Ok(Vec::<IOWarrior>::with_capacity(0)),
        Some(x) => x,
    };

    let device_count = unsafe { iowkit.IowKitGetNumDevs() };
    let mut vec: Vec<IOWarrior> = Vec::new();

    let iowkit_data = Arc::new(IowkitData {
        iowkit,
        iowkit_handle,
    });

    for index in 0..device_count {
        let device_handle =
            match NonNull::new(unsafe { iowkit_data.iowkit.IowKitGetDeviceHandle(index + 1) }) {
                None => continue,
                Some(x) => x,
            };

        let device_product_id = unsafe {
            iowkit_data
                .iowkit
                .IowKitGetProductId(device_handle.as_ptr())
        } as u16;

        let device_revision =
            unsafe { iowkit_data.iowkit.IowKitGetRevision(device_handle.as_ptr()) } as u16;

        let device_type = match IOWarriorType::from_device_product_id(device_product_id) {
            None => continue,
            Some(x) => x,
        };

        if device_type == IOWarriorType::IOWarrior40 && device_revision < 0x1010 {
            continue;
        }

        let device_serial = {
            let mut raw_device_serial_number = [0u16; 9];

            let device_serial_number_result = unsafe {
                iowkit_data.iowkit.IowKitGetSerialNumber(
                    device_handle.as_ptr(),
                    raw_device_serial_number.as_mut_ptr(),
                )
            };

            if device_serial_number_result > 0i32 {
                String::from_utf16_lossy(&raw_device_serial_number)
            } else {
                return Err(InitializationError::InternalError(
                    "Failed to get serial number.".to_owned(),
                ));
            }
        };

        let communication_data = CommunicationData {
            iowkit_data: iowkit_data.clone(),
            device_handle,
            max_pipe: get_max_pipe(device_type),
        };

        let iowarrior = iowarrior_service::create_iowarrior(
            device_type,
            device_revision,
            device_serial,
            communication_data,
        )
        .map_err(|x| InitializationError::ErrorUSB(x))?;

        vec.push(iowarrior);
    }

    Ok(vec)
}

fn get_max_pipe(device_type: IOWarriorType) -> u8 {
    match device_type {
        IOWarriorType::IOWarrior28
        | IOWarriorType::IOWarrior28Dongle
        | IOWarriorType::IOWarrior100 => Pipe::ADCMode.get_value(),
        IOWarriorType::IOWarrior40
        | IOWarriorType::IOWarrior24
        | IOWarriorType::IOWarrior24PowerVampire
        | IOWarriorType::IOWarrior28L
        | IOWarriorType::IOWarrior56
        | IOWarriorType::IOWarrior56Dongle => Pipe::SpecialMode.get_value(),
    }
}

pub fn get_iowarrior(serial_number: &str) -> Result<IOWarrior, InitializationError> {
    match get_iowarriors()?
        .into_iter()
        .filter(|x| x.get_serial_number() == serial_number)
        .next()
    {
        None => Err(InitializationError::NotFound(String::from(serial_number))),
        Some(x) => Ok(x),
    }
}
