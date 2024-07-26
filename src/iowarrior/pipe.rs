use crate::backend::PipeImpl;
use crate::iowarrior::{HidError, IOWarriorLock, PipeName, Report};
use std::fmt;
use std::sync::Arc;

// SAFETY:
// Keep _iowarrior_lock always after pipe_impl in this struct.
// Variables are dropped in their order of declaration.
// pipe_impl must have always been dropped before _iowarrior_lock.
#[derive(Debug)]
pub struct Pipe {
    report_size: usize,
    pipe_name: PipeName,
    pipe_impl: PipeImpl,
    _iowarrior_lock: Arc<IOWarriorLock>,
}

impl Pipe {
    pub fn new(
        pipe_impl: PipeImpl,
        pipe_name: PipeName,
        iowarrior_lock: Arc<IOWarriorLock>,
        report_size: usize,
    ) -> Pipe {
        Pipe {
            report_size,
            pipe_impl,
            pipe_name,
            _iowarrior_lock: iowarrior_lock,
        }
    }

    pub fn create_report(&self) -> Report {
        Report {
            buffer: vec![0u8; self.report_size],
            pipe: self.pipe_name,
        }
    }

    pub fn write_report(&mut self, report: &Report) -> Result<(), HidError> {
        if report.buffer.len() != self.report_size {
            return Err(HidError::InvalidReportSizeData);
        }

        let bytes_written = self.pipe_impl.write_report(report.buffer.as_slice())?;

        if bytes_written != report.buffer.len() {
            return Err(HidError::IncompleteSendError {
                sent: bytes_written,
                all: report.buffer.len(),
            });
        }

        Ok(())
    }

    pub fn read_report_non_blocking(&mut self, report: &mut Report) -> Result<bool, HidError> {
        if report.buffer.len() != self.report_size {
            return Err(HidError::InvalidReportSizeData);
        }

        let bytes_read = self
            .pipe_impl
            .read_report_non_blocking(report.buffer.as_mut_slice())?;

        if bytes_read > 0 && bytes_read != report.buffer.len() {
            return Err(HidError::IncompleteReceiveError {
                received: bytes_read,
                all: report.buffer.len(),
            });
        }

        Ok(bytes_read > 0)
    }

    pub fn read_report(&mut self, report: &mut Report) -> Result<(), HidError> {
        if report.buffer.len() != self.report_size {
            return Err(HidError::InvalidReportSizeData);
        }

        let bytes_read = self.pipe_impl.read_report(report.buffer.as_mut_slice())?;

        if bytes_read != report.buffer.len() {
            return Err(HidError::IncompleteReceiveError {
                received: bytes_read,
                all: report.buffer.len(),
            });
        }

        Ok(())
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
