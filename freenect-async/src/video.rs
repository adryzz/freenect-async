use crate::context::{FreenectReadyAll, FreenectReadyVideo, FreenectReadyVideoMotors};

pub trait FreenectVideo {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}

impl FreenectVideo for FreenectReadyAll {}
