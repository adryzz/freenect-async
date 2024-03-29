pub mod context;
pub mod device;
pub mod formats;
pub mod motors_led;
pub mod stream;
pub mod video;

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum FreenectError {
    #[error("Unable to create the freenect context.")]
    ContextCreationError,
    #[error("Unable to list connected freenect devices.")]
    DeviceListError,
    #[error("Device {0} not found.")]
    DeviceNotFound(u32),
    #[error("Unable to open device {0}.")]
    OpenDeviceError(u32),
    #[error("Unable to set LED state.")]
    LedStateError,
    #[error("A tilt angle of {0}° is out of range! It should be between ±31°.")]
    TiltAngleOutOfRange(f64),
    #[error("Unable to set tilt angle.")]
    TiltAngleError,
    #[error("A brightness value of {0} is out of range! It should be between 1 and 50.")]
    BrightnessOutOfRange(u16),
    #[error("Unable to set brightness value.")]
    SetBrightnessError,
    #[error("Unable to get brightness value.")]
    GetBrightnessError,
    #[error("Error while processing events")]
    EventProcessingError,
    #[error("Error with the video stream.")]
    VideoStreamError,
    #[error("Bad video format")]
    BadVideoFormat,
}
