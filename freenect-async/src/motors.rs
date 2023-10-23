use crate::{device::FreenectDevice, context::{FreenectDeviceReady, FreenectDeviceMode, FreenectReadyVideoMotors, FreenectReadyMotors}, FreenectError};

pub trait FreenectMotors {}

impl FreenectMotors for FreenectReadyVideoMotors {}

impl FreenectMotors for FreenectReadyMotors {}

impl<'a, D> FreenectDevice<'a, D> where D: FreenectDeviceReady + FreenectDeviceMode + FreenectMotors {
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

/// FIXME: add options
pub struct FreenectTiltState;

impl FreenectTiltState {
    fn to_c(&self) -> freenect_sys::freenect_raw_tilt_state {
        todo!()
    }
}
