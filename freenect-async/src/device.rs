use crate::context::{FreenectDeviceReady, FreenectDeviceMode, FreenectContext};

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