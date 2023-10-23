use crate::context::{FreenectReadyVideo, FreenectReadyVideoMotors};

pub trait FreenectVideo {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}
