# iowarrior-embedded-hal
A Rust library that provides access to the IOWarrior GPIO, I2C, PWM, SPI, and ADC peripherals.

Both embedded-hal v0.2.7 and v1 are supported.

## Backends

There are three different backends available to interact with the IOWarrior boards. The backends can be selected as a crate feature:

1. **iowkit**: This is the default backend, written in C, provided by Code Mercenaries. The iowkit library is dynamically loaded at runtime. You have to 'install' the official IOWarrior-SDK. This backend is supported by Windows and Linux.


2. **usbhid**: This is an experimental backend and is only available on Windows. It is a pure Rust implementation and interacts directly with the WIN32 HID API.


3. **ioctrl**: This is also an experimental backend and is only available on Linux. It is a pure Rust implementation and interacts directly with the ioctrl kernel interface.

## License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br>

## Linux Prerequisites

Example for Ubuntu 24.04, other distros may differ:
```
# Auto load iowarrior kernel module at boot
echo 'iowarrior' | sudo tee /etc/modules-load.d/iowarrior.conf

# Allow dialout user group to access IOWarrior devices
echo 'KERNEL=="iowarrior*", NAME="usb/iowarrior%n", OWNER="root", GROUP="dialout", MODE="0666"' | sudo tee /etc/udev/rules.d/99-iowarrior.rules

# Repeat for every user: Add user X to dialout group
sudo usermod -a -G dialout X

# Reboot to let changes take place
sudo reboot
```
