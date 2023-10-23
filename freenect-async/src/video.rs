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
        unsafe {
            let res = freenect_sys::freenect_set_ir_brightness(self.inner, brightness);
            if res < 0 {
                return Err(FreenectError::SetBrightnessError);
            }
        }

        Ok(())
    }
}
