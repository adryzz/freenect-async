use std::{fmt, mem::transmute};

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
        depth: &FreenectVideoMode,
    ) -> Result<VideoStream<'a, 'b, D>, FreenectError> {

        if let FreenectFormat::Depth(_) = video.format {
            return Err(FreenectError::BadVideoFormat);
        }
        if let FreenectFormat::Video(_) = depth.format {
            return Err(FreenectError::BadVideoFormat);
        }

        unsafe {
            let dev = self.inner;
            if freenect_sys::freenect_set_video_mode(dev, video.into()) < 0 {
                return Err(FreenectError::BadVideoFormat);
            }
            if freenect_sys::freenect_set_depth_mode(dev, depth.into()) < 0 {
                return Err(FreenectError::BadVideoFormat);
            }

            let video = VideoStream { device: self };
            freenect_sys::freenect_set_user(dev, transmute(&video));
            freenect_sys::freenect_set_video_callback(dev, Some(video_callback));
            freenect_sys::freenect_set_depth_callback(dev, Some(depth_callback));

            if freenect_sys::freenect_start_depth(dev) < 0 {
                return Err(FreenectError::VideoStreamError);
            }

            if freenect_sys::freenect_start_video(dev) < 0 {
                return Err(FreenectError::VideoStreamError);
            }
            Ok(video)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FreenectVideoMode {
    _reserved: u32,
    pub format: FreenectFormat,
    pub resolution: FreenectResolution,
    pub bytes: u32,
    pub width: u16,
    pub height: u16,
    pub data_bits_per_pixel: u8,
    pub padding_bits_per_pixel: u8,
    pub framerate: u8,
    pub is_valid: bool,
}

impl From<&FreenectVideoMode> for freenect_sys::freenect_frame_mode {
    fn from(value: &FreenectVideoMode) -> Self {
        freenect_sys::freenect_frame_mode {
            reserved: value._reserved,
            resolution: value.resolution as u32,
            __bindgen_anon_1: match value.format {
                FreenectFormat::Video(v) => freenect_sys::freenect_frame_mode__bindgen_ty_1 {
                    video_format: v as u32,
                },
                FreenectFormat::Depth(d) => freenect_sys::freenect_frame_mode__bindgen_ty_1 {
                    depth_format: d as u32,
                },
            },
            bytes: value.bytes as i32,
            width: value.width as i16,
            height: value.height as i16,
            data_bits_per_pixel: value.data_bits_per_pixel as i8,
            padding_bits_per_pixel: value.padding_bits_per_pixel as i8,
            framerate: value.framerate as i8,
            is_valid: if value.is_valid { 1 } else { 0 },
        }
    }
}

impl fmt::Display for FreenectVideoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mbps = (self.bytes * self.framerate as u32) / (1024 * 1024 / 8);
        write!(
            f,
            "{} {}x{}@{}fps {}bpp, {}Mbps",
            self.format, self.width, self.height, self.framerate, self.data_bits_per_pixel, mbps
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FreenectFormat {
    Video(FreenectVideoFormat),
    Depth(FreenectDepthFormat),
}

impl fmt::Display for FreenectFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreenectFormat::Video(v) => write!(f, "Video ({})", v),
            FreenectFormat::Depth(d) => write!(f, "Depth ({})", d),
        }
    }
}

impl From<FreenectVideoFormat> for FreenectFormat {
    fn from(value: FreenectVideoFormat) -> Self {
        FreenectFormat::Video(value)
    }
}

impl From<FreenectDepthFormat> for FreenectFormat {
    fn from(value: FreenectDepthFormat) -> Self {
        FreenectFormat::Depth(value)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum FreenectDepthFormat {
    Depth10Bit = freenect_sys::freenect_depth_format_FREENECT_DEPTH_10BIT,
    Depth10BitPacked = freenect_sys::freenect_depth_format_FREENECT_DEPTH_10BIT_PACKED,
    #[default]
    Depth11Bit = freenect_sys::freenect_depth_format_FREENECT_DEPTH_11BIT,
    Depth11BitPacked = freenect_sys::freenect_depth_format_FREENECT_DEPTH_11BIT_PACKED,
    DepthRegistered = freenect_sys::freenect_depth_format_FREENECT_DEPTH_REGISTERED,
    DepthMillimeters = freenect_sys::freenect_depth_format_FREENECT_DEPTH_MM,
    //DepthDummy = freenect_sys::freenect_depth_format_FREENECT_DEPTH_DUMMY
}

impl fmt::Display for FreenectDepthFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreenectDepthFormat::Depth10Bit => write!(f, "10bit"),
            FreenectDepthFormat::Depth10BitPacked => write!(f, "10bit packed"),
            FreenectDepthFormat::Depth11Bit => write!(f, "11bit"),
            FreenectDepthFormat::Depth11BitPacked => write!(f, "11bit packed"),
            FreenectDepthFormat::DepthRegistered => write!(f, "registered"),
            FreenectDepthFormat::DepthMillimeters => write!(f, "mm"),
        }
    }
}

impl TryFrom<u32> for FreenectDepthFormat {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_10BIT => {
                FreenectDepthFormat::Depth10Bit
            }
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_10BIT_PACKED => {
                FreenectDepthFormat::Depth10BitPacked
            }
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_11BIT => {
                FreenectDepthFormat::Depth11Bit
            }
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_11BIT_PACKED => {
                FreenectDepthFormat::Depth11BitPacked
            }
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_REGISTERED => {
                FreenectDepthFormat::DepthRegistered
            }
            freenect_sys::freenect_depth_format_FREENECT_DEPTH_MM => {
                FreenectDepthFormat::DepthMillimeters
            }
            _ => return Err(()),
        })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum FreenectVideoFormat {
    #[default]
    Rgb = freenect_sys::freenect_video_format_FREENECT_VIDEO_RGB,
    YuvRgb = freenect_sys::freenect_video_format_FREENECT_VIDEO_YUV_RGB,
    YuvRaw = freenect_sys::freenect_video_format_FREENECT_VIDEO_YUV_RAW,
    Bayer = freenect_sys::freenect_video_format_FREENECT_VIDEO_BAYER,
    Ir8Bit = freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_8BIT,
    Ir10Bit = freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_10BIT,
    Ir10BitPacked = freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_10BIT_PACKED,
    //Dummy = freenect_sys::freenect_video_format_FREENECT_VIDEO_DUMMY
}

impl fmt::Display for FreenectVideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreenectVideoFormat::Rgb => write!(f, "RGB"),
            FreenectVideoFormat::YuvRgb => write!(f, "YUV-RGB"),
            FreenectVideoFormat::YuvRaw => write!(f, "YUV-RAW"),
            FreenectVideoFormat::Bayer => write!(f, "BAYER"),
            FreenectVideoFormat::Ir8Bit => write!(f, "IR-8bit"),
            FreenectVideoFormat::Ir10Bit => write!(f, "IR-10bit"),
            FreenectVideoFormat::Ir10BitPacked => write!(f, "IR-10bit packed"),
        }
    }
}

impl TryFrom<u32> for FreenectVideoFormat {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            freenect_sys::freenect_video_format_FREENECT_VIDEO_RGB => FreenectVideoFormat::Rgb,
            freenect_sys::freenect_video_format_FREENECT_VIDEO_YUV_RGB => {
                FreenectVideoFormat::YuvRgb
            }
            freenect_sys::freenect_video_format_FREENECT_VIDEO_YUV_RAW => {
                FreenectVideoFormat::YuvRaw
            }
            freenect_sys::freenect_video_format_FREENECT_VIDEO_BAYER => FreenectVideoFormat::Bayer,
            freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_8BIT => {
                FreenectVideoFormat::Ir8Bit
            }
            freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_10BIT => {
                FreenectVideoFormat::Ir10Bit
            }
            freenect_sys::freenect_video_format_FREENECT_VIDEO_IR_10BIT_PACKED => {
                FreenectVideoFormat::Ir10BitPacked
            }
            _ => return Err(()),
        })
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum FreenectResolution {
    #[default]
    Low = freenect_sys::freenect_resolution_FREENECT_RESOLUTION_LOW,
    Medium = freenect_sys::freenect_resolution_FREENECT_RESOLUTION_MEDIUM,
    High = freenect_sys::freenect_resolution_FREENECT_RESOLUTION_HIGH, //Dummy = freenect_sys::freenect_resolution_FREENECT_RESOLUTION_DUMMY
}

impl TryFrom<u32> for FreenectResolution {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            freenect_sys::freenect_resolution_FREENECT_RESOLUTION_LOW => FreenectResolution::Low,
            freenect_sys::freenect_resolution_FREENECT_RESOLUTION_MEDIUM => {
                FreenectResolution::Medium
            }
            freenect_sys::freenect_resolution_FREENECT_RESOLUTION_HIGH => FreenectResolution::High,
            _ => return Err(()),
        })
    }
}

extern "C" fn depth_callback<'a>(
    dev: *mut freenect_sys::freenect_device,
    data: *mut std::os::raw::c_void,
    timestamp: u32,
) {
    unsafe {
        let data = data as *mut u16;
        let data = std::slice::from_raw_parts(data, 640 * 480);
        let device =
            freenect_sys::freenect_get_user(dev) as *mut FreenectDevice<'a, FreenectReadyVideo>;
        let device = &*device;
        println!("Depth");
    }
}

extern "C" fn video_callback<'a>(
    dev: *mut freenect_sys::freenect_device,
    data: *mut std::os::raw::c_void,
    timestamp: u32,
) {
    unsafe {
        let data = data as *mut u8;
        let data = std::slice::from_raw_parts(data, 640 * 480 * 3);
        let device =
            freenect_sys::freenect_get_user(dev) as *mut FreenectDevice<'a, FreenectReadyVideo>;
        let device = &*device;
        println!("Video");
    }
}

pub struct VideoStream<'a, 'b, D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo> {
    // keep this private
    device: &'b mut FreenectDevice<'a, D>,
}

impl<'a, 'b, D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo> VideoStream<'a, 'b, D> {
    pub fn dev_ref(&'b self) -> &'b FreenectDevice<'a, D> {
        self.device
    }

    pub async fn try_read_depth_frame(&mut self) -> Result<Option<DepthFrame>, FreenectError> {
        unsafe {
            let res = freenect_sys::freenect_process_events_timeout(
                self.device.context.inner,
                std::mem::zeroed(),
            );
            if res < 0 {
                return Err(FreenectError::EventProcessingError);
            }
        }
        todo!()
    }

    pub async fn try_read_camera_frame(&mut self) -> Result<Option<CameraFrame>, FreenectError> {
        unsafe {
            let res = freenect_sys::freenect_process_events_timeout(
                self.device.context.inner,
                std::mem::zeroed(),
            );
            if res < 0 {
                return Err(FreenectError::EventProcessingError);
            }
        }
        todo!()
    }
}

impl<'a, 'b, D: FreenectDeviceReady + FreenectDeviceMode + FreenectVideo> Drop
    for VideoStream<'a, 'b, D>
{
    fn drop(&mut self) {
        unsafe {
            freenect_sys::freenect_set_video_callback(self.device.inner, None);
            freenect_sys::freenect_set_depth_callback(self.device.inner, None);
            freenect_sys::freenect_set_user(self.device.inner, std::ptr::null_mut());
        }
    }
}

pub struct DepthFrame<'c> {
    timestamp: u32,
    data: &'c [u16],
}

pub struct CameraFrame<'c> {
    timestamp: u32,
    data: &'c [u8],
}
