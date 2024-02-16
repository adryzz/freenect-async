use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct FreenectVideoMode {
    pub(crate) _reserved: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
