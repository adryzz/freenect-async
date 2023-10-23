use std::{mem::MaybeUninit, ptr};

use thiserror::Error;

pub struct FreenectContext<M: FreenectDeviceMode> {
    inner: *mut freenect_sys::freenect_context,

    marker: std::marker::PhantomData<M>,
}

impl FreenectContext<FreenectInitialized> {
    pub fn new() -> Result<FreenectContext<FreenectInitialized>, FreenectError> {
        unsafe {
            #[allow(invalid_value)]
            let mut inner = MaybeUninit::uninit().assume_init();
            let res = freenect_sys::freenect_init(&mut inner, ptr::null_mut());
            #[warn(invalid_value)]
            if res < 0 {
                return Err(FreenectError::ContextCreationError);
            }
            Ok(Self {
                inner,
                marker: std::marker::PhantomData,
            })
        }
    }

    pub fn setup_video(self) -> FreenectContext<FreenectReadyVideo> {
        unsafe {
            freenect_sys::freenect_select_subdevices(
                self.inner,
                freenect_sys::freenect_device_flags_FREENECT_DEVICE_CAMERA,
            )
        };
        FreenectContext {
            inner: self.inner,
            marker: std::marker::PhantomData,
        }
    }

    pub fn setup_video_motors(self) -> FreenectContext<FreenectReadyVideoMotors> {
        unsafe {
            freenect_sys::freenect_select_subdevices(
                self.inner,
                freenect_sys::freenect_device_flags_FREENECT_DEVICE_CAMERA
                    | freenect_sys::freenect_device_flags_FREENECT_DEVICE_MOTOR,
            )
        };
        FreenectContext {
            inner: self.inner,
            marker: std::marker::PhantomData,
        }
    }
}

impl<M> FreenectContext<M> where M : FreenectDeviceMode {
    pub fn list_devices(&self) -> Result<u32, FreenectError> {
        let res = unsafe { freenect_sys::freenect_num_devices(self.inner) };
        if res < 0 {
            return Err(FreenectError::DeviceListError);
        }
        Ok(res as u32)
    }
}

/*impl<M> FreenectContext<M> where M : FreenectDeviceReady {
    pub fn open_device(&self) -> Result<u32, FreenectError> {

    }
}*/

pub trait FreenectDeviceMode {}

struct FreenectInitialized {}

impl FreenectDeviceMode for FreenectInitialized {}

pub trait FreenectDeviceReady {}

struct FreenectReadyVideo {}

impl FreenectDeviceMode for FreenectReadyVideo {}

impl FreenectDeviceReady for FreenectReadyVideo {}

struct FreenectReadyVideoMotors {}

impl FreenectDeviceMode for FreenectReadyVideoMotors {}

impl FreenectDeviceReady for FreenectReadyVideoMotors {}

#[derive(Debug, Clone, Copy, Error)]
pub enum FreenectError {
    #[error("Unable to create the freenect context.")]
    ContextCreationError,
    #[error("Unable to list connected freenect devices.")]
    DeviceListError,
    #[error("Device {0} not found.")]
    DeviceNotFound(usize),
    #[error("Unable to open device.")]
    OpenDeviceError,
}
