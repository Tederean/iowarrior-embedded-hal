use std::ffi::CString;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum HidError {
    #[error("HID API error: {message}")]
    HidApiError { message: String },

    #[error("HID API error.")]
    HidApiErrorEmpty,

    #[error("Initialization error.")]
    InitializationError,

    #[error("Invalid zero size data.")]
    InvalidZeroSizeData,

    #[error("Incomplete send error. Send {sent}, expected {all}.")]
    IncompleteSendError { sent: usize, all: usize },

    #[error("Incomplete receive error. Received {received}, expected {all}.")]
    IncompleteReceiveError { received: usize, all: usize },

    #[error("Set blocking mode error: {mode}")]
    SetBlockingModeError { mode: &'static str },

    #[error("Opened HID device with device error info.")]
    OpenHidDeviceWithDeviceInfoError {
        path: CString,
        vendor_id: u16,
        product_id: u16,
        interface_number: u8,
    },

    #[error("Invalid report size data.")]
    InvalidReportSizeData,

    #[error("IO error: {error}")]
    IoError { error: std::io::Error },
}
