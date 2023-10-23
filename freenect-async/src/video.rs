use crate::context::{FreenectReadyVideo, FreenectReadyVideoMotors, FreenectReadyAll};

pub trait FreenectVideo {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}

impl FreenectVideo for FreenectReadyAll {}