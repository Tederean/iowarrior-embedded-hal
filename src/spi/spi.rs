use crate::iowarrior::Peripheral;
use crate::iowarrior::{peripheral_service, IOWarriorData, IOWarriorMutData};
use crate::spi::{spi_service, SPIConfig, SPIData, SPIError};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug)]
pub struct SPI {
    pub(crate) data: Rc<IOWarriorData>,
    pub(crate) mut_data_refcell: Rc<RefCell<IOWarriorMutData>>,
    pub(crate) spi_data: SPIData,
}

impl fmt::Display for SPI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Drop for SPI {
    #[inline]
    fn drop(&mut self) {
        peripheral_service::disable_peripheral(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            Peripheral::SPI,
        );
    }
}

impl embedded_hal::spi::ErrorType for SPI {
    type Error = SPIError;
}

impl embedded_hal::spi::SpiBus<u8> for SPI {
    #[inline]
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        spi_service::read_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            words,
        )
    }

    #[inline]
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        spi_service::write_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            words,
        )
    }

    #[inline]
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        spi_service::transfer_data_with_different_size(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            read,
            write,
        )
    }

    #[inline]
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        spi_service::transfer_data_in_place(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            words,
        )
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::spi::Transfer<u8> for SPI {
    type Error = SPIError;

    #[inline]
    fn transfer<'a>(&mut self, buffer: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        let write_buffer = buffer.to_vec();

        spi_service::transfer_data_with_same_size(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            buffer,
            write_buffer.as_slice(),
        )?;

        Ok(buffer)
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::spi::Write<u8> for SPI {
    type Error = SPIError;

    #[inline]
    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        spi_service::write_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            buffer,
        )
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::spi::WriteIter<u8> for SPI {
    type Error = SPIError;

    #[inline]
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        let write = words.into_iter().collect::<Vec<u8>>();

        spi_service::write_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            write.as_slice(),
        )
    }
}

#[cfg(feature = "embedded-hal-0")]
impl embedded_hal_0::blocking::spi::Transactional<u8> for SPI {
    type Error = SPIError;

    #[inline]
    fn exec<'a>(
        &mut self,
        operations: &mut [embedded_hal_0::blocking::spi::Operation<'a, u8>],
    ) -> Result<(), Self::Error> {
        let mut mut_data = self.mut_data_refcell.borrow_mut();

        for operation in operations {
            match operation {
                embedded_hal_0::blocking::spi::Operation::Write(write) => {
                    spi_service::write_data(&self.data, &mut mut_data, &self.spi_data, write)?;
                }
                embedded_hal_0::blocking::spi::Operation::Transfer(transfer) => {
                    spi_service::transfer_data_in_place(
                        &self.data,
                        &mut mut_data,
                        &self.spi_data,
                        transfer,
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl embedded_hal::spi::SpiDevice for SPI {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        let mut mut_data = self.mut_data_refcell.borrow_mut();

        for operation in operations {
            match operation {
                embedded_hal::spi::Operation::Read(read) => {
                    spi_service::read_data(&self.data, &mut mut_data, &self.spi_data, read)?;
                }
                embedded_hal::spi::Operation::Write(write) => {
                    spi_service::write_data(&self.data, &mut mut_data, &self.spi_data, write)?;
                }
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    spi_service::transfer_data_with_different_size(
                        &self.data,
                        &mut mut_data,
                        &self.spi_data,
                        read,
                        write,
                    )?;
                }
                embedded_hal::spi::Operation::TransferInPlace(buf) => {
                    spi_service::transfer_data_in_place(
                        &self.data,
                        &mut mut_data,
                        &self.spi_data,
                        buf,
                    )?;
                }
                embedded_hal::spi::Operation::DelayNs(delay_ns) => {
                    std::thread::sleep(Duration::from_nanos(delay_ns.clone() as u64));
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        spi_service::read_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            buf,
        )
    }

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        spi_service::write_data(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            buf,
        )
    }

    #[inline]
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        spi_service::transfer_data_with_different_size(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            read,
            write,
        )
    }

    #[inline]
    fn transfer_in_place(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        spi_service::transfer_data_in_place(
            &self.data,
            &mut self.mut_data_refcell.borrow_mut(),
            &self.spi_data,
            buf,
        )
    }
}

impl SPI {
    #[inline]
    pub fn get_config(&self) -> SPIConfig {
        self.spi_data.spi_config.clone()
    }

    #[inline]
    pub fn get_frequency_hz(&self) -> u32 {
        self.spi_data.calculated_frequency_hz
    }
}
