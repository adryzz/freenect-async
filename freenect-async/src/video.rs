use crate::context::{FreenectReadyVideoMotors, FreenectReadyVideo};


pub trait FreenectVideo {}

impl FreenectVideo for FreenectReadyVideo {}

impl FreenectVideo for FreenectReadyVideoMotors {}