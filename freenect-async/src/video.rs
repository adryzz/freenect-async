use std::marker::PhantomData;

use crate::{
    context::{
        FreenectDeviceMode, FreenectDeviceReady, FreenectReadyAll, FreenectReadyVideo,
        FreenectReadyVideoMotors,
    },
    device::FreenectDevice,
    FreenectError,
};

const MAX_IR_BRIGHTNESS: u16 = 50;
const MIN_IR_BRIGHTNESS: u16 = 50;

pub trait FreenectVideo {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}

impl FreenectVideo for FreenectReadyAll {}

impl<'a, D> FreenectDevice<'a, D>
where
    D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo,
{
    pub fn get_ir_brightness(&self) -> Result<u16, FreenectError> {
        unsafe {
            let res = freenect_sys::freenect_get_ir_brightness(self.inner);
            if res < 0 {
                return Err(FreenectError::GetBrightnessError);
            }

            Ok(res as u16)
        }
    }

    pub fn set_ir_brightness(&mut self, brightness: u16) -> Result<(), FreenectError> {
        if brightness > MAX_IR_BRIGHTNESS || brightness < MIN_IR_BRIGHTNESS {
            return Err(FreenectError::BrightnessOutOfRange(brightness));
        }
        unsafe {
            let res = freenect_sys::freenect_set_ir_brightness(self.inner, brightness);
            if res < 0 {
                return Err(FreenectError::SetBrightnessError);
            }
        }

        Ok(())
    }

    pub fn start_video_stream<'b>(&'b mut self) -> Result<VideoStream<'a, 'b, D>, FreenectError> {
        Ok(VideoStream { device: self })
    }
}

pub struct VideoStream<'a, 'b, D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo> {
    // keep this private
    device: &'b mut FreenectDevice<'a, D>
}

impl<'a, 'b, D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo> VideoStream<'a, 'b, D> {
    
    pub fn dev_ref(&'b self) -> &'b FreenectDevice<'a, D> {
        self.device
    }

    pub async fn try_read_depth_frame(&mut self) -> Result<Option<DepthFrame>, FreenectError> {
        unsafe { 
            let res = freenect_sys::freenect_process_events_timeout(self.device.context.inner, std::mem::zeroed());
        }
        todo!()
    }

    pub async fn try_read_camera_frame(&mut self) -> Result<Option<CameraFrame>, FreenectError> {
        unsafe { 
            let res = freenect_sys::freenect_process_events_timeout(self.device.context.inner, std::mem::zeroed());
        }
        todo!()
    }
}

pub struct DepthFrame<'c> {
    timestamp: u32,
    data: &'c [u16]
}

pub struct CameraFrame<'c> {
    timestamp: u32,
    data: &'c [u8]
}