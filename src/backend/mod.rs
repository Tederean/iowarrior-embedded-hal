mod shared;

#[allow(unused_imports)]
pub use self::shared::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "hid")] {

        mod hid;
        pub use self::hid::*;

    } else if #[cfg(feature = "usb")] {

        mod usb;
        pub use self::usb::*;

    } else if #[cfg(feature = "iowkit")] {

        mod iowkit;
        pub use self::iowkit::*;

    } else {
        compile_error!("No backend (crate feature) selected.");
    }
}
