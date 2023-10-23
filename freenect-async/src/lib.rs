pub mod context;
pub mod device;
pub mod motors_led;
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
}
