use crate::iowarrior::{HidError, IOWarriorType};
use iowkit_sys::{Iowkit, PCHAR, ULONG};
use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::fmt;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

static IOWKIT_LIBRARY: Mutex<Weak<LibraryContainer>> = Mutex::new(Weak::new());

#[derive(Debug)]
struct LibraryContainer {
    library: Arc<Iowkit>,
    library_handle: Option<NonNull<c_void>>,
    device_handles: Vec<NonNull<c_void>>,
}

unsafe impl Sync for LibraryContainer {}
unsafe impl Send for LibraryContainer {}

impl Drop for LibraryContainer {
    #[inline]
    fn drop(&mut self) {
        match self.library_handle {
            None => {}
            Some(library_handle) => unsafe {
                self.library.IowKitCloseDevice(library_handle.as_ptr())
            },
        }
    }
}

fn get_iowkit_library(
    mut mutex_guard: MutexGuard<Weak<LibraryContainer>>,
) -> Result<Arc<LibraryContainer>, HidError> {
    match mutex_guard.upgrade() {
        None => {
            let path = format!("{}{}{}", DLL_PREFIX, "iowkit", DLL_SUFFIX);

            let library =
                Arc::new(unsafe { Iowkit::new(path) }.map_err(|_| HidError::InitializationError)?);

            let library_handle = NonNull::new(unsafe { library.IowKitOpenDevice() });

            let device_count = unsafe { library.IowKitGetNumDevs() };

            let device_handles = (0..device_count)
                .into_iter()
                .filter_map(|x| NonNull::new(unsafe { library.IowKitGetDeviceHandle(x + 1) }))
                .collect();

            let arc = Arc::new(LibraryContainer {
                library,
                library_handle,
                device_handles,
            });

            *mutex_guard = Arc::downgrade(&arc);

            Ok(arc)
        }
        Some(library) => Ok(library),
    }
}

#[derive(Debug, Clone)]
pub struct PipeInfo {
    library_container: Arc<LibraryContainer>,
    device_handle: NonNull<c_void>,
    interface: u8,
    product_id: u16,
    device_serial: Option<String>,
}

impl PipeInfo {
    pub fn collect() -> Result<Vec<PipeInfo>, HidError> {
        let mutex_guard = IOWKIT_LIBRARY.lock().unwrap();

        let library_container = get_iowkit_library(mutex_guard)?;

        let mut pipes = Vec::new();

        for handle in &library_container.device_handles {
            let product_id = unsafe {
                library_container
                    .library
                    .IowKitGetProductId(handle.as_ptr())
            } as u16;

            let pipe_count = match IOWarriorType::from_device_product_id(product_id) {
                None => continue,
                Some(x) => x.pipe_count(),
            };

            let device_serial = {
                let mut raw_device_serial_number = [0u16; 9];

                let device_serial_number_result = unsafe {
                    library_container.library.IowKitGetSerialNumber(
                        handle.as_ptr(),
                        raw_device_serial_number.as_mut_ptr(),
                    )
                };

                if device_serial_number_result > 0i32 {
                    Some(String::from_utf16_lossy(&raw_device_serial_number))
                } else {
                    None
                }
            };

            for interface in 0..pipe_count {
                pipes.push(PipeInfo {
                    library_container: library_container.clone(),
                    device_handle: handle.clone(),
                    interface,
                    product_id,
                    device_serial: device_serial.clone(),
                });
            }
        }

        Ok(pipes)
    }

    pub fn pipe(&self) -> u8 {
        self.interface
    }

    pub fn serial_number(&self) -> Option<&str> {
        self.device_serial.as_ref().map(|s| &**s)
    }

    pub fn product_id(&self) -> u16 {
        self.product_id
    }

    pub fn open(self) -> Result<PipeImpl, HidError> {
        Ok(PipeImpl {
            library_container: self.library_container,
            device_handle: self.device_handle,
            interface: self.interface,
        })
    }
}

impl fmt::Display for PipeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct PipeImpl {
    library_container: Arc<LibraryContainer>,
    device_handle: NonNull<c_void>,
    interface: u8,
}

impl PipeImpl {
    pub fn write_report(&mut self, report: &[u8]) -> Result<usize, HidError> {
        let written_bytes = unsafe {
            self.library_container.library.IowKitWrite(
                self.device_handle.as_ptr(),
                self.interface as ULONG,
                report.as_ptr() as PCHAR,
                report.len() as ULONG,
            )
        } as usize;

        Ok(written_bytes)
    }

    pub fn read_report_non_blocking(&mut self, report: &mut [u8]) -> Result<usize, HidError> {
        let read_bytes = unsafe {
            self.library_container.library.IowKitReadNonBlocking(
                self.device_handle.as_ptr(),
                self.interface as ULONG,
                report.as_mut_ptr() as PCHAR,
                report.len() as ULONG,
            )
        } as usize;

        Ok(read_bytes)
    }

    pub fn read_report(&mut self, report: &mut [u8]) -> Result<usize, HidError> {
        let read_bytes = unsafe {
            self.library_container.library.IowKitRead(
                self.device_handle.as_ptr(),
                self.interface as ULONG,
                report.as_mut_ptr() as PCHAR,
                report.len() as ULONG,
            )
        } as usize;

        Ok(read_bytes)
    }

    pub fn revision(&mut self) -> Result<Option<u16>, HidError> {
        let revision = unsafe {
            self.library_container
                .library
                .IowKitGetRevision(self.device_handle.as_ptr())
        } as u16;

        Ok(Some(revision))
    }
}

impl fmt::Display for PipeImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
