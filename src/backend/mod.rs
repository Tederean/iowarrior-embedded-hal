cfg_if::cfg_if! {
    if #[cfg(feature = "usbhid")] {

        mod usbhid;
        pub(crate) use self::usbhid::*;

    } else if #[cfg(feature = "iowkit")] {
        compile_error!("iowkit is not implemented yet.");
    } else {
        compile_error!("No backend selected. Enable one of the following features: usbhid, iowkit");
    }
}
