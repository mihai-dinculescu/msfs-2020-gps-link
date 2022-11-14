use simconnect_sdk::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
pub struct GpsData {
    #[simconnect(name = "PLANE LATITUDE", unit = "Degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "Degrees")]
    pub lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "Meters")]
    pub alt: f64,
    #[simconnect(name = "PRESSURE ALTITUDE", unit = "Meters")]
    pub pressure_altitude: f64,
    #[simconnect(name = "GPS GROUND MAGNETIC TRACK", unit = "Degrees")]
    pub gps_ground_magnetic_track: f64,
    #[simconnect(name = "MAGVAR", unit = "Degrees")]
    pub gps_magnetic_variation: f64,
    #[simconnect(name = "GPS GROUND SPEED", unit = "Meters per second")]
    pub gps_ground_speed: f64,
}

impl GpsData {
    pub fn lat_as_degrees_minutes_dir(&self) -> (f64, f64, &str) {
        let abs = self.lat.abs();
        let deg = abs.trunc();
        let min = abs.fract() * 60.0;
        let dir = if self.lat >= 0.0 { "N" } else { "S" };

        (deg, min, dir)
    }

    pub fn lon_as_degrees_minutes_dir(&self) -> (f64, f64, &str) {
        let abs = self.lon.abs();
        let deg = abs.trunc();
        let min = abs.fract() * 60.0;
        let dir = if self.lon >= 0.0 { "E" } else { "W" };

        (deg, min, dir)
    }

    pub fn gps_magnetic_variation_as_abs_dir(&self) -> (f64, &str) {
        let dir = if self.gps_magnetic_variation >= 0.0 {
            "E"
        } else {
            "W"
        };

        let abs = self.gps_magnetic_variation.abs();

        (abs, dir)
    }

    pub fn gps_ground_speed_in_knots(&self) -> f64 {
        self.gps_ground_speed * 1.9438444924574
    }
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed")]
pub struct OnGround {
    #[simconnect(name = "SIM ON GROUND")]
    pub sim_on_ground: bool,
}
