use simconnect_sdk::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
pub struct GpsData {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "meters")]
    pub alt: f64,
    #[simconnect(name = "GPS GROUND MAGNETIC TRACK", unit = "Degrees")]
    pub gps_ground_magnetic_track: f64,
    #[simconnect(name = "MAGVAR", unit = "Degrees")]
    pub gps_magnetic_variation: f64,
    #[simconnect(name = "GPS GROUND SPEED", unit = "Meters per second")]
    pub gps_ground_speed: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed")]
pub struct OnGround {
    #[simconnect(name = "SIM ON GROUND")]
    pub sim_on_ground: bool,
}
