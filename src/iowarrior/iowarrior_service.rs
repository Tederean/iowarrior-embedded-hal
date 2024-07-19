use crate::backend::PipeInfo;
use crate::iowarrior::{
    HidError, IOWarrior, IOWarriorData, IOWarriorInfo, IOWarriorLock, IOWarriorMutData,
    IOWarriorType, Pipe, PipeName, Report, ReportId,
};
use itertools::Itertools;
use std::sync::{Arc, Mutex};

pub fn get_iowarriors() -> Result<Vec<IOWarriorInfo>, HidError> {
    let all_pipes = PipeInfo::collect()?;

    let pipe_map = all_pipes
        .into_iter()
        .filter_map(|x| {
            let device_serial = match x.serial_number() {
                None => return None,
                Some(x) => x.to_string(),
            };

            Some((x, device_serial))
        })
        .into_group_map_by(|x| x.1.clone());

    Ok(pipe_map
        .into_iter()
        .filter_map(|(device_serial, mut pipes)| {
            pipes.sort_by_key(|x| x.0.pipe());
            pipes.reverse();

            let pipe_0 = match pipes.pop() {
                None => return None,
                Some(x) => match x.0.pipe() {
                    0 => x.0,
                    _ => return None,
                },
            };

            let pipe_1 = match pipes.pop() {
                None => return None,
                Some(x) => match x.0.pipe() {
                    1 => x.0,
                    _ => return None,
                },
            };

            let pipe_2 = match pipes.pop() {
                None => None,
                Some(x) => match x.0.pipe() {
                    2 => Some(x.0),
                    _ => return None,
                },
            };

            let pipe_3 = match pipes.pop() {
                None => None,
                Some(x) => match x.0.pipe() {
                    3 => Some(x.0),
                    _ => return None,
                },
            };

            let device_type = match IOWarriorType::from_device_product_id(pipe_0.product_id()) {
                None => return None,
                Some(x) => x,
            };

            match (&pipe_2, &pipe_3, device_type.pipe_count()) {
                (Some(_), Some(_), 4) => {}
                (None, None, 2) => {}
                _ => return None,
            };

            let possible_device_types = match device_type {
                IOWarriorType::IOWarrior40
                | IOWarriorType::IOWarrior24PowerVampire
                | IOWarriorType::IOWarrior28L
                | IOWarriorType::IOWarrior100 => vec![device_type],
                IOWarriorType::IOWarrior24 | IOWarriorType::IOWarrior24Dongle => {
                    vec![IOWarriorType::IOWarrior24, IOWarriorType::IOWarrior24Dongle]
                }
                IOWarriorType::IOWarrior28 | IOWarriorType::IOWarrior28Dongle => {
                    vec![IOWarriorType::IOWarrior28, IOWarriorType::IOWarrior28Dongle]
                }
                IOWarriorType::IOWarrior56 | IOWarriorType::IOWarrior56Dongle => {
                    vec![IOWarriorType::IOWarrior56, IOWarriorType::IOWarrior56Dongle]
                }
            };

            Some(IOWarriorInfo::new(
                pipe_0,
                pipe_1,
                pipe_2,
                pipe_3,
                device_serial,
                device_type,
                possible_device_types,
            ))
        })
        .collect())
}

pub(crate) fn open_iowarrior(
    pipe_info_0: PipeInfo,
    pipe_info_1: PipeInfo,
    pipe_info_2: Option<PipeInfo>,
    pipe_info_3: Option<PipeInfo>,
    device_serial: String,
    device_type: IOWarriorType,
) -> Result<IOWarrior, HidError> {
    let iowarrior_lock = Arc::new(IOWarriorLock::new(device_serial.clone())?);

    let mut pipe_impl_0 = pipe_info_0.open()?;
    let pipe_impl_1 = pipe_info_1.open()?;

    let pipe_impl_2 = match pipe_info_2 {
        None => None,
        Some(x) => Some(x.open()?),
    };

    let pipe_impl_3 = match pipe_info_3 {
        None => None,
        Some(x) => Some(x.open()?),
    };

    let device_revision = pipe_impl_0.revision()?;

    let standard_report_size = get_standard_report_size(device_type);
    let special_report_size = get_special_report_size(device_type);

    let pipe_0 = Pipe::new(
        pipe_impl_0,
        PipeName::IOPins,
        iowarrior_lock.clone(),
        standard_report_size,
    );
    let mut pipe_1 = Pipe::new(
        pipe_impl_1,
        PipeName::SpecialMode,
        iowarrior_lock.clone(),
        special_report_size,
    );
    let pipe_2 = pipe_impl_2.map(|x| {
        Pipe::new(
            x,
            PipeName::I2CMode,
            iowarrior_lock.clone(),
            special_report_size,
        )
    });
    let mut pipe_3 = pipe_impl_3.map(|x| {
        Pipe::new(
            x,
            PipeName::ADCMode,
            iowarrior_lock.clone(),
            special_report_size,
        )
    });

    let mut data = IOWarriorData {
        device_serial,
        device_revision,
        device_type,
        standard_report_size,
        special_report_size,
    };

    if data.device_type == IOWarriorType::IOWarrior24 && is_dongle(&mut pipe_1, ReportId::IrSetup)? {
        data.device_type = IOWarriorType::IOWarrior24Dongle;
    }

    else if data.device_type == IOWarriorType::IOWarrior56 && is_dongle(&mut pipe_1, ReportId::AdcSetup)? {
        data.device_type = IOWarriorType::IOWarrior56Dongle;
    }

    else if data.device_type == IOWarriorType::IOWarrior28 && is_dongle(pipe_3.as_mut().unwrap(), ReportId::AdcSetup)? {
        data.device_type = IOWarriorType::IOWarrior28Dongle;
    }

    let pins_report = get_pins_report(&data, &mut pipe_1)?;

    let mut_data = IOWarriorMutData {
        pins_in_use: vec![],
        dangling_peripherals: vec![],
        pins_write_report: pins_report.clone(),
        pins_read_report: pins_report,
        pipe_0,
        pipe_1,
        pipe_2,
        pipe_3,
    };

    Ok(IOWarrior {
        data: Arc::new(data),
        mut_data_mutex: Arc::new(Mutex::new(mut_data)),
    })
}

#[inline]
fn get_standard_report_size(device_type: IOWarriorType) -> usize {
    match device_type {
        IOWarriorType::IOWarrior24
        | IOWarriorType::IOWarrior24Dongle
        | IOWarriorType::IOWarrior24PowerVampire => 3,
        IOWarriorType::IOWarrior28
        | IOWarriorType::IOWarrior28Dongle
        | IOWarriorType::IOWarrior28L
        | IOWarriorType::IOWarrior40 => 5,
        IOWarriorType::IOWarrior56
        | IOWarriorType::IOWarrior56Dongle
        | IOWarriorType::IOWarrior100 => 8,
    }
}

#[inline]
fn get_special_report_size(device_type: IOWarriorType) -> usize {
    match device_type {
        IOWarriorType::IOWarrior40
        | IOWarriorType::IOWarrior24
        | IOWarriorType::IOWarrior24Dongle
        | IOWarriorType::IOWarrior24PowerVampire
        | IOWarriorType::IOWarrior28L => 8,
        IOWarriorType::IOWarrior28
        | IOWarriorType::IOWarrior28Dongle
        | IOWarriorType::IOWarrior56
        | IOWarriorType::IOWarrior56Dongle
        | IOWarriorType::IOWarrior100 => 64,
    }
}

fn is_dongle(pipe: &mut Pipe, report_id: ReportId) -> Result<bool, HidError> {
    let mut report = pipe.create_report();

    report.buffer[0] = report_id.get_value();
    report.buffer[1] = 0x00;

    match pipe.write_report(&mut report) {
        Ok(_) => Ok(false),
        Err(error) => {
            match error {
                HidError::IncompleteSendError { sent, all: _ } => {
                    if sent == 0 {
                        return Ok(true);
                    }
                }
                _ => {}
            }

            Err(error)
        }
    }
}

#[inline]
fn get_pins_report(data: &IOWarriorData, pipe_1_special: &mut Pipe) -> Result<Report, HidError> {
    {
        let mut report = pipe_1_special.create_report();

        report.buffer[0] = ReportId::GpioSpecialRead.get_value();

        pipe_1_special.write_report(&report)?;
    }

    {
        let mut report = pipe_1_special.create_report();

        pipe_1_special.read_report(&mut report)?;

        report.buffer[0] = ReportId::GpioReadWrite.get_value();

        Ok(Report {
            pipe: PipeName::IOPins,
            buffer: report
                .buffer
                .iter()
                .map(|x| *x)
                .take(data.standard_report_size)
                .collect(),
        })
    }
}
