pub mod adc;
pub(crate) mod backend;
pub mod bits;
pub mod delay;
pub mod digital;
pub mod i2c;
pub mod iowarrior;
pub mod pwm;
pub mod spi;

pub use self::iowarrior::iowarrior_service::get_iowarriors;

#[macro_export]
macro_rules! pin {
    ($n:expr, $m:expr) => {
        8 * $n + $m
    };
}
