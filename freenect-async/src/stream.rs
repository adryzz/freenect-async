use lending_stream::LendingStream;
use std::{future::Future, task::Poll};

use crate::{
    context::{FreenectDeviceMode, FreenectDeviceReady, FreenectReadyVideo}, device::FreenectDevice, formats::{FreenectFormat, FreenectVideoMode}, video::FreenectVideo, FreenectError
};

#[derive(Debug)]
pub struct VideoStream<'a, 'b, D: FreenectVideo> {
    // keep this private
    pub(crate) device: &'b mut FreenectDevice<'a, D>,
    pub(crate) counter: u32,
    pub(crate) out: Option<(&'b [u8], u32)>
}

impl<'a, 'b, D: FreenectVideo> VideoStream<'a, 'b, D> {
    pub(crate) fn new(device: &'b mut FreenectDevice<'a, D>, video: &FreenectVideoMode) -> Result<Self, FreenectError> {
        if let FreenectFormat::Depth(_) = video.format {
            return Err(FreenectError::BadVideoFormat);
        }

        unsafe {
            let dev = device.inner;
            if freenect_sys::freenect_set_video_mode(dev, video.into()) < 0 {
                return Err(FreenectError::BadVideoFormat);
            }
            freenect_sys::freenect_start_video(dev);
            freenect_sys::freenect_set_video_callback(dev, Some(video_callback_standalone));

            let stream = Self {
                device: device,
                counter: 0,
                out: None
            };

            freenect_sys::freenect_set_user(dev, std::mem::transmute(&stream));

            Ok(stream)
        }
    }

    pub fn dev_ref(&'b self) -> &'b FreenectDevice<'a, D> {
        &self.device
    }
}

extern "C" fn video_callback_standalone<'a>(
    dev: *mut freenect_sys::freenect_device,
    data: *mut std::os::raw::c_void,
    timestamp: u32,
) {
    unsafe {
        let data = data as *mut u8;
        let data = std::slice::from_raw_parts(data, 640 * 480 * 3);
        let device =
            freenect_sys::freenect_get_user(dev) as *mut VideoStream<'a, 'a, FreenectReadyVideo>;
        let device = &mut *device;

        device.out = Some((data, timestamp));
        println!("Video");
    }
}

impl<'a, 'b, D: FreenectVideo> Drop
    for VideoStream<'a, 'b, D>
{
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_stop_video(self.device.inner);
            freenect_sys::freenect_set_video_callback(self.device.inner, None);
            freenect_sys::freenect_set_user(self.device.inner, std::ptr::null_mut());
        }
    }
}

impl<'a, 'b, D: FreenectVideo> LendingStream
    for VideoStream<'a, 'b, D>
{
    type Item<'c> = Result<CameraFrame<'a, 'b, 'c, D>, FreenectError> where Self: 'c;

    fn poll_next(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item<'_>>> {
        // retrieve frame if available
        if let Some((data, timestamp)) = self.out {
            self.out = None;
            let frame = CameraFrame {
                _held: self,
                timestamp,
                data
            };
            return Poll::Ready(Some(Ok(frame)))
        }


        let res = unsafe { freenect_sys::freenect_process_events(self.device.context.inner) };
        if res < 0 {
            self.counter = 0;
            return Poll::Ready(Some(Err(FreenectError::EventProcessingError)));
        }
        if self.counter == 0 {

            self.counter += 1;
        }

        return Poll::Pending;
    }
}

#[derive(Debug)]
pub struct DepthStream<'a, 'b, D: FreenectVideo> {
    // keep this private
    device: &'b mut FreenectDevice<'a, D>,
    pub(crate) counter: u32,
    pub(crate) out: Option<(&'b [u16], u32)>
}

impl<'a, 'b, D: FreenectVideo> DepthStream<'a, 'b, D> {
    pub(crate) fn new(device: &'b mut FreenectDevice<'a, D>, video: &FreenectVideoMode) -> Result<Self, FreenectError> {
        if let FreenectFormat::Video(_) = video.format {
            return Err(FreenectError::BadVideoFormat);
        }

        unsafe {
            let dev = device.inner;
            if freenect_sys::freenect_set_depth_mode(dev, video.into()) < 0 {
                return Err(FreenectError::BadVideoFormat);
            }
            freenect_sys::freenect_start_depth(dev);
            freenect_sys::freenect_set_depth_callback(dev, Some(depth_callback_standalone));

            let stream = Self {
                device: device,
                counter: 0,
                out: None
            };

            freenect_sys::freenect_set_user(dev, std::mem::transmute(&stream));

            Ok(stream)
        }
    }

    pub fn dev_ref(&'b self) -> &'b FreenectDevice<'a, D> {
        &self.device
    }
}

extern "C" fn depth_callback_standalone<'a>(
    dev: *mut freenect_sys::freenect_device,
    data: *mut std::os::raw::c_void,
    timestamp: u32,
) {
    unsafe {
        let data = data as *mut u16;
        let data = std::slice::from_raw_parts(data, 640 * 480);
        let device =
            freenect_sys::freenect_get_user(dev) as *mut DepthStream<'a, 'a, FreenectReadyVideo>;
        let device = &mut *device;

        device.out = Some((data, timestamp));
        println!("Depth");
    }
}

impl<'a, 'b, D: FreenectVideo> Drop
    for DepthStream<'a, 'b, D>
{
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_stop_depth(self.device.inner);
            freenect_sys::freenect_set_depth_callback(self.device.inner, None);
            freenect_sys::freenect_set_user(self.device.inner, std::ptr::null_mut());
        }
    }
}

pub struct VideoDepthStream<'a, 'b, D: FreenectVideo> {
    // keep this private
    device: &'b mut FreenectDevice<'a, D>,
}

impl<'a, 'b, D: FreenectVideo> Drop
    for VideoDepthStream<'a, 'b, D>
{
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_stop_video(self.device.inner);
            freenect_sys::freenect_stop_depth(self.device.inner);
            freenect_sys::freenect_set_video_callback(self.device.inner, None);
            freenect_sys::freenect_set_depth_callback(self.device.inner, None);
            freenect_sys::freenect_set_user(self.device.inner, std::ptr::null_mut());
        }
    }
}

impl<'a, 'b, D: FreenectVideo> LendingStream
    for DepthStream<'a, 'b, D>
{
    type Item<'c> = Result<DepthFrame<'a, 'b, 'c, D>, FreenectError> where Self: 'c;

    fn poll_next(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item<'_>>> {
        // retrieve frame if available
        if let Some((data, timestamp)) = self.out {
            self.out = None;
            let frame = DepthFrame {
                _held: self,
                timestamp,
                data
            };
            return Poll::Ready(Some(Ok(frame)))
        }

        if self.counter == 0 {
            let res = unsafe { freenect_sys::freenect_process_events(self.device.context.inner) };
            if res < 0 {
                self.counter = 0;
                return Poll::Ready(Some(Err(FreenectError::EventProcessingError)));
            }

            self.counter += 1;
        }

        return Poll::Pending;
    }
}

#[derive(Debug)]
pub struct DepthFrame<'a, 'b, 'c, D: FreenectVideo> {
    _held: &'c DepthStream<'a, 'b, D>,
    pub timestamp: u32,
    pub data: &'c [u16],
}

#[derive(Debug)]
pub struct CameraFrame<'a, 'b, 'c, D: FreenectVideo> {
    _held: &'c VideoStream<'a, 'b, D>,
    pub timestamp: u32,
    pub data: &'c [u8],
}
