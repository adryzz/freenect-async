use std::{
    ffi::CStr,
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
            if freenect_sys::freenect_init(inner.as_mut_ptr(), ptr::null_mut()) < 0 {
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

    pub fn set_log_level(&self, level: FreenectLogLevel) {
        unsafe {
            freenect_sys::freenect_set_log_level(self.inner, level as u32);
        }
    }

    pub fn set_log_callback(&self, callback: Option<LogCallback>) {
        unsafe extern "C" fn c_callback_wrapper(
            _dev: *mut freenect_sys::freenect_context,
            level: freenect_sys::freenect_loglevel,
            msg: *const std::os::raw::c_char,
        ) {
            let msg_str = std::ffi::CStr::from_ptr(msg);
            let level = FreenectLogLevel::try_from(level).unwrap_or_default();
            if let Some(cb) = LOG_CALLBACK {
                cb(level, msg_str)
            }
        }

        unsafe {
            match callback {
                None => freenect_sys::freenect_set_log_callback(self.inner, None),
                Some(c) => {
                    freenect_sys::freenect_set_log_callback(self.inner, Some(c_callback_wrapper));
                    LOG_CALLBACK = Some(c);
                }
            }
        }
    }

    fn into_handle(self) -> *mut freenect_sys::freenect_context {
        let m = ManuallyDrop::new(self);
        m.inner
    }
}

type LogCallback = fn(level: FreenectLogLevel, msg: &CStr);
// FIXME: find a way to not use a static mut here
static mut LOG_CALLBACK: Option<LogCallback> = None;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FreenectLogLevel {
    #[default]
    Fatal = freenect_sys::freenect_loglevel_FREENECT_LOG_FATAL,
    Error = freenect_sys::freenect_loglevel_FREENECT_LOG_ERROR,
    Warning = freenect_sys::freenect_loglevel_FREENECT_LOG_WARNING,
    Notice = freenect_sys::freenect_loglevel_FREENECT_LOG_NOTICE,
    Info = freenect_sys::freenect_loglevel_FREENECT_LOG_INFO,
    Debug = freenect_sys::freenect_loglevel_FREENECT_LOG_DEBUG,
    Spew = freenect_sys::freenect_loglevel_FREENECT_LOG_SPEW,
    Flood = freenect_sys::freenect_loglevel_FREENECT_LOG_FLOOD,
}

impl TryFrom<u32> for FreenectLogLevel {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        Ok(match value {
            freenect_sys::freenect_loglevel_FREENECT_LOG_FATAL => FreenectLogLevel::Fatal,
            freenect_sys::freenect_loglevel_FREENECT_LOG_ERROR => FreenectLogLevel::Error,
            freenect_sys::freenect_loglevel_FREENECT_LOG_WARNING => FreenectLogLevel::Warning,
            freenect_sys::freenect_loglevel_FREENECT_LOG_NOTICE => FreenectLogLevel::Notice,
            freenect_sys::freenect_loglevel_FREENECT_LOG_INFO => FreenectLogLevel::Info,
            freenect_sys::freenect_loglevel_FREENECT_LOG_DEBUG => FreenectLogLevel::Debug,
            freenect_sys::freenect_loglevel_FREENECT_LOG_SPEW => FreenectLogLevel::Spew,
            freenect_sys::freenect_loglevel_FREENECT_LOG_FLOOD => FreenectLogLevel::Flood,
            _ => return Err(()),
        })
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
