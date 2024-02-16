use std::mem::ManuallyDrop;

use crate::context::{FreenectContext, FreenectDeviceReady};

#[derive(Debug)]
pub struct FreenectDevice<'a, D: FreenectDeviceReady> {
    pub context: &'a mut FreenectContext<D>,
    pub(crate) inner: *mut freenect_sys::freenect_device,
    pub(crate) marker: std::marker::PhantomData<D>,
}

impl<'a, D: FreenectDeviceReady> Drop for FreenectDevice<'a, D> {
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_close_device(self.inner);
        }
    }
}

impl<'a, D: FreenectDeviceReady> FreenectDevice<'a, D> {
    fn into_handle(self) -> *mut freenect_sys::freenect_device {
        let m = ManuallyDrop::new(self);
        m.inner
    }
}
