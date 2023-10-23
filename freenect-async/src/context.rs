use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ptr,
};

use crate::{device::FreenectDevice, FreenectError};

pub trait FreenectDeviceMode {}

pub enum FreenectInitialized {}

impl FreenectDeviceMode for FreenectInitialized {}

pub trait FreenectDeviceReady {}

pub enum FreenectReadyVideo {}

impl FreenectDeviceMode for FreenectReadyVideo {}

impl FreenectDeviceReady for FreenectReadyVideo {}

pub enum FreenectReadyVideoMotors {}

impl FreenectDeviceMode for FreenectReadyVideoMotors {}

impl FreenectDeviceReady for FreenectReadyVideoMotors {}

pub enum FreenectReadyMotors {}

impl FreenectDeviceMode for FreenectReadyMotors {}

impl FreenectDeviceReady for FreenectReadyMotors {}

pub enum FreenectReadyAll {}

impl FreenectDeviceMode for FreenectReadyAll {}

impl FreenectDeviceReady for FreenectReadyAll {}

pub struct FreenectContext<M: FreenectDeviceMode> {
    pub(crate) inner: *mut freenect_sys::freenect_context,

    pub(crate) marker: std::marker::PhantomData<M>,
}

impl FreenectContext<FreenectInitialized> {
    pub fn new() -> Result<Self, FreenectError> {
        unsafe {
            let mut inner = MaybeUninit::uninit();
            let res = freenect_sys::freenect_init(inner.as_mut_ptr(), ptr::null_mut());
            if res < 0 {
                return Err(FreenectError::ContextCreationError);
            }
            let inner = inner.assume_init();
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
            inner: self.into_handle(),
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
            inner: self.into_handle(),
            marker: std::marker::PhantomData,
        }
    }

    pub fn setup_motors(self) -> FreenectContext<FreenectReadyMotors> {
        unsafe {
            freenect_sys::freenect_select_subdevices(
                self.inner,
                freenect_sys::freenect_device_flags_FREENECT_DEVICE_MOTOR,
            )
        };
        FreenectContext {
            inner: self.into_handle(),
            marker: std::marker::PhantomData,
        }
    }

    pub fn setup_all(self) -> FreenectContext<FreenectReadyAll> {
        // do not call freenect_select_subdevices, as all subdevices are selected by default
        FreenectContext {
            inner: self.into_handle(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<M> FreenectContext<M>
where
    M: FreenectDeviceMode,
{
    pub fn list_devices(&self) -> Result<u32, FreenectError> {
        let res = unsafe { freenect_sys::freenect_num_devices(self.inner) };
        if res < 0 {
            return Err(FreenectError::DeviceListError);
        }
        Ok(res as u32)
    }

    fn into_handle(self) -> *mut freenect_sys::freenect_context {
        let m = ManuallyDrop::new(self);
        m.inner
    }
}

impl<M> FreenectContext<M>
where
    M: FreenectDeviceReady + FreenectDeviceMode,
{
    pub fn open_device(&mut self, index: u32) -> Result<FreenectDevice<M>, FreenectError> {
        if index >= self.list_devices()? {
            return Err(FreenectError::DeviceNotFound(index));
        }
        unsafe {
            let mut dev = MaybeUninit::uninit();
            if freenect_sys::freenect_open_device(self.inner, dev.as_mut_ptr(), index as i32) < 0 {
                return Err(FreenectError::OpenDeviceError(index));
            }
            let dev = dev.assume_init();
            Ok(FreenectDevice {
                inner: dev,
                marker: self.marker,
                context: self,
            })
        }
    }
}

impl<M: FreenectDeviceMode> Drop for FreenectContext<M> {
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_shutdown(self.inner);
        }
    }
}
