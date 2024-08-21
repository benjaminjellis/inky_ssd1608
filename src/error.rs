use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::SPIError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InkyError {
    #[error(transparent)]
    SPI(#[from] SPIError),
    #[error("Invalid Colour")]
    InvaidColour,
    #[error("Invalid Resolution")]
    InvalidResolution,
    #[error("Spi config: `{0}`")]
    SpiConfig(String),
    #[error(transparent)]
    LinuxI2C(#[from] LinuxI2CError),
    #[error(transparent)]
    GPIO(#[from] linux_embedded_hal::sysfs_gpio::Error),
}
