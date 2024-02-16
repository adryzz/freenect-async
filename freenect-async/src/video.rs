use std::{fmt, mem::{transmute, MaybeUninit}};

use crate::{
    context::{
        FreenectDeviceMode, FreenectDeviceReady, FreenectReadyAll, FreenectReadyVideo,
        FreenectReadyVideoMotors,
    }, device::FreenectDevice, formats::{FreenectDepthFormat, FreenectFormat, FreenectResolution, FreenectVideoFormat, FreenectVideoMode}, stream::{DepthStream, VideoDepthStream, VideoStream}, FreenectError
};

const MAX_IR_BRIGHTNESS: u16 = 50;
const MIN_IR_BRIGHTNESS: u16 = 50;

pub trait FreenectVideo: FreenectDeviceReady {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}

impl FreenectVideo for FreenectReadyAll {}

impl<'a, D> FreenectDevice<'a, D>
where
    D: FreenectVideo,
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
            if freenect_sys::freenect_set_ir_brightness(self.inner, brightness) < 0 {
                return Err(FreenectError::SetBrightnessError);
            }
        }

        Ok(())
    }

    pub fn get_supported_video_modes(&self) -> Vec<FreenectVideoMode> {
        unsafe {
            let count = freenect_sys::freenect_get_video_mode_count() as usize;
            let mut modes: Vec<FreenectVideoMode> = Vec::with_capacity(count);

            for i in 0..count {
                let pre_mode = freenect_sys::freenect_get_video_mode(i as i32);
                if let Ok(format) =
                    FreenectVideoFormat::try_from(pre_mode.__bindgen_anon_1.video_format)
                {
                    if let Ok(resolution) = FreenectResolution::try_from(pre_mode.resolution) {
                        let mode = FreenectVideoMode {
                            _reserved: pre_mode.reserved,
                            format: format.into(),
                            resolution,
                            bytes: pre_mode.bytes as u32,
                            width: pre_mode.width as u16,
                            height: pre_mode.height as u16,
                            data_bits_per_pixel: pre_mode.data_bits_per_pixel as u8,
                            padding_bits_per_pixel: pre_mode.padding_bits_per_pixel as u8,
                            framerate: pre_mode.framerate as u8,
                            is_valid: pre_mode.is_valid == 1,
                        };
                        modes.push(mode);
                    }
                }
            }
            modes
        }
    }

    pub fn get_supported_depth_modes(&self) -> Vec<FreenectVideoMode> {
        unsafe {
            let count = freenect_sys::freenect_get_depth_mode_count() as usize;
            let mut modes: Vec<FreenectVideoMode> = Vec::with_capacity(count);

            for i in 0..count {
                let pre_mode = freenect_sys::freenect_get_depth_mode(i as i32);
                if let Ok(format) =
                    FreenectDepthFormat::try_from(pre_mode.__bindgen_anon_1.depth_format)
                {
                    if let Ok(resolution) = FreenectResolution::try_from(pre_mode.resolution) {
                        let mode = FreenectVideoMode {
                            _reserved: pre_mode.reserved,
                            format: format.into(),
                            resolution,
                            bytes: pre_mode.bytes as u32,
                            width: pre_mode.width as u16,
                            height: pre_mode.height as u16,
                            data_bits_per_pixel: pre_mode.data_bits_per_pixel as u8,
                            padding_bits_per_pixel: pre_mode.padding_bits_per_pixel as u8,
                            framerate: pre_mode.framerate as u8,
                            is_valid: pre_mode.is_valid == 1,
                        };
                        modes.push(mode);
                    }
                }
            }
            modes
        }
    }

    pub fn start_video_stream<'b>(
        &'b mut self,
        video: &FreenectVideoMode,
    ) -> Result<VideoStream<'a, 'b, D>, FreenectError> {

        VideoStream::new(self, video)
    }

    pub fn start_depth_stream<'b>(
        &'b mut self,
        depth: &FreenectVideoMode,
    ) -> Result<DepthStream<'a, 'b, D>, FreenectError> {
        DepthStream::new(self, depth)
    }

    pub fn start_video_depth_stream<'b>(
        &'b mut self,
        depth: &FreenectVideoMode,
    ) -> Result<VideoDepthStream<'a, 'b, D>, FreenectError> {
        todo!()
    }
}
