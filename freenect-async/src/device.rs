use std::{default, mem::ManuallyDrop};

use crate::{
    context::{FreenectContext, FreenectDeviceMode, FreenectDeviceReady},
    FreenectError,
};

pub struct FreenectDevice<'a, D: FreenectDeviceReady + FreenectDeviceMode> {
    pub context: &'a FreenectContext<D>,
    pub(crate) inner: *mut freenect_sys::freenect_device,
    pub(crate) marker: std::marker::PhantomData<D>,
}

impl<'a, D: FreenectDeviceReady + FreenectDeviceMode> Drop for FreenectDevice<'a, D> {
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_close_device(self.inner);
        }
    }
}

impl<'a, D: FreenectDeviceReady + FreenectDeviceMode> FreenectDevice<'a, D> {
    fn into_handle(self) -> *mut freenect_sys::freenect_device {
        let m = ManuallyDrop::new(self);
        m.inner
    }
}
