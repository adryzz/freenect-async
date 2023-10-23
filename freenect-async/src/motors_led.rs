use crate::{
    context::{
        FreenectDeviceMode, FreenectDeviceReady, FreenectReadyMotors, FreenectReadyVideoMotors, FreenectReadyAll,
    },
    device::FreenectDevice,
    FreenectError,
};

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
            let res = freenect_sys::freenect_set_led(self.inner, state as u32);
            if res < 0 {
                return Err(FreenectError::LedStateError);
            }
        }

        Ok(())
    }

    pub fn set_tilt_degree(&self, deg: f64) -> Result<(), FreenectError> {
        Ok(())
    }

    pub fn get_tilt_degree(&self) -> Result<f64, FreenectError> {
        Ok(0.0)
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

impl FreenectTiltState {
    fn to_c(&self) -> freenect_sys::freenect_raw_tilt_state {
        todo!()
    }
}
