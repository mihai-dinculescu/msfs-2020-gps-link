use std::{fmt, io};

use crate::system::simconnect_objects::GpsData;

pub trait BroadcasterExt: fmt::Debug {
    fn send(&mut self, data: GpsData) -> Result<(), io::Error>;
}
