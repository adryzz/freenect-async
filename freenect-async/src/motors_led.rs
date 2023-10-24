use crate::{
    context::{
        FreenectDeviceMode, FreenectDeviceReady, FreenectReadyAll, FreenectReadyMotors,
        FreenectReadyVideoMotors,
    },
    device::FreenectDevice,
    FreenectError,
};

const MIN_TILT_ANGLE: f64 = -31.0;
const MAX_TILT_ANGLE: f64 = 31.0;

pub trait FreenectMotors {}

impl FreenectMotors for FreenectReadyVideoMotors {}

impl FreenectMotors for FreenectReadyMotors {}

impl FreenectMotors for FreenectReadyAll {}

impl<'a, D> FreenectDevice<'a, D>
where
    D: FreenectDeviceReady + FreenectDeviceMode + FreenectMotors,
{
    pub fn set_led(&self, state: FreenectLedState) -> Result<(), FreenectError> {
        unsafe {
            if freenect_sys::freenect_set_led(self.inner, state as u32) < 0 {
                return Err(FreenectError::LedStateError);
            }
        }

        Ok(())
    }

    pub fn set_tilt_degree(&self, deg: f64) -> Result<(), FreenectError> {
        if deg > MAX_TILT_ANGLE || deg < MIN_TILT_ANGLE {
            return Err(FreenectError::TiltAngleOutOfRange(deg));
        }
        unsafe {
            if freenect_sys::freenect_set_tilt_degs(self.inner, deg) < 0 {
                return Err(FreenectError::TiltAngleError);
            }
        }
        Ok(())
    }

    pub fn get_tilt_degree(&self) -> Result<f64, FreenectError> {
        todo!()
    }

    pub fn get_tilt_state(&self) -> Result<FreenectTiltState, FreenectError> {
        Ok(FreenectTiltState)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FreenectLedState {
    Off = freenect_sys::freenect_led_options_LED_OFF,
    #[default]
    Green = freenect_sys::freenect_led_options_LED_GREEN,
    Red = freenect_sys::freenect_led_options_LED_RED,
    Yellow = freenect_sys::freenect_led_options_LED_YELLOW,
    BlinkGreen = freenect_sys::freenect_led_options_LED_BLINK_GREEN,
    BlinkRedYellow = freenect_sys::freenect_led_options_LED_BLINK_RED_YELLOW,
}

/// FIXME: add options
pub struct FreenectTiltState;
