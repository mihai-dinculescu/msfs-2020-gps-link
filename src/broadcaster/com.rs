use std::{fmt, io, time::SystemTime};

use chrono::{DateTime, Utc};
use serialport::SerialPort;
use tracing::{error, instrument, trace, warn};

use crate::system::simconnect_objects::GpsData;

use super::{BroadcasterExt, ComConfig};

const NMEA_MID_GSA_MESSAGE: &str =
    "$GPGSA,A,3,01,02,03,04,05,06,07,08,09,10,11,12,1.0,1.0,1.0*30\r\n";
const NMEA_MID_GSA_INTERVAL_S: u64 = 1;

pub struct Com {
    port: Box<dyn SerialPort>,
    last_mid_gsa: Option<SystemTime>,
}

impl fmt::Debug for Com {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Com")
            .field("port", &"...")
            .field("last_mid_gsa", &self.last_mid_gsa)
            .finish()
    }
}

impl Com {
    #[instrument(name = "Com::new")]
    pub fn new(config: ComConfig) -> Result<Box<dyn BroadcasterExt>, io::Error> {
        let port = serialport::new(config.port, config.baud_rate)
            .open()
            .map_err(|e| {
                error!(error = ?e, "failed to open COM port");
                e
            })?;

        Ok(Box::new(Com {
            port,
            last_mid_gsa: None,
        }))
    }
}

impl BroadcasterExt for Com {
    #[instrument(name = "Com::send", skip(self, data))]
    fn send(&mut self, data: GpsData) -> Result<(), io::Error> {
        if self.should_send_mid_gsa() {
            self.write(NMEA_MID_GSA_MESSAGE)?;
            self.last_mid_gsa.replace(SystemTime::now());
        }

        let date = Utc::now();

        let message = Self::convert_gps_data_to_nmea_mid_gga(&date, &data);
        self.write(&message)?;

        let message = Self::convert_gps_data_to_nmea_mid_rmc(&date, &data);
        self.write(&message)?;

        trace!("Successfully sent broadcast over COM");

        Ok(())
    }
}

impl Com {
    fn should_send_mid_gsa(&self) -> bool {
        match self.last_mid_gsa {
            Some(last_mid_gsa) => {
                if let Ok(elapsed) = last_mid_gsa.elapsed() {
                    if elapsed.as_secs() >= NMEA_MID_GSA_INTERVAL_S {
                        return true;
                    }
                }
            }
            None => {
                return true;
            }
        }

        false
    }

    fn write(&mut self, data: &str) -> Result<(), io::Error> {
        match self.port.write(data.as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                warn!(error = ?e, "failed to write to COM (timeout)")
            }
            Err(e) => {
                error!(error = ?e, "failed to write to COM");
                return Err(e);
            }
        };

        Ok(())
    }

    fn convert_gps_data_to_nmea_mid_gga(date: &DateTime<Utc>, data: &GpsData) -> String {
        let lat = data.lat.abs();
        let lat_deg = lat.trunc();
        let lat_min = lat.fract() * 60.0;
        let lat_dir = if data.lat >= 0.0 { "N" } else { "S" };

        let lon = data.lon.abs();
        let lon_deg = lon.trunc();
        let lon_min = lon.fract() * 60.0;
        let lon_dir = if data.lon >= 0.0 { "E" } else { "W" };

        let message = format!(
            "$GPGGA,{},{:0>2}{:0>7.4},{},{:0>3}{:0>7.4},{},1,12,1.0,{:.1},M,0.0,M,,",
            date.format("%H%M%S%.3f"),
            lat_deg,
            lat_min,
            lat_dir,
            lon_deg,
            lon_min,
            lon_dir,
            data.pressure_altitude
        );

        let checksum = message.chars().skip(1).fold(0u8, |acc, c| acc ^ c as u8);

        format!("{message}*{checksum:X}\r\n")
    }

    fn convert_gps_data_to_nmea_mid_rmc(date: &DateTime<Utc>, data: &GpsData) -> String {
        let track = data.gps_ground_magnetic_track - data.gps_magnetic_variation;

        let lat = data.lat.abs();
        let lat_deg = lat.trunc();
        let lat_min = lat.fract() * 60.0;
        let lat_dir = if data.lat >= 0.0 { "N" } else { "S" };

        let lon = data.lon.abs();
        let lon_deg = lon.trunc();
        let lon_min = lon.fract() * 60.0;
        let lon_dir = if data.lon >= 0.0 { "E" } else { "W" };

        let message = format!(
            "$GPRMC,{},A,{:0>2}{:0>7.4},{},{:0>3}{:0>7.4},{},{:.1},{:.1},{},,,S",
            date.format("%H%M%S%.3f"),
            lat_deg,
            lat_min,
            lat_dir,
            lon_deg,
            lon_min,
            lon_dir,
            data.gps_ground_speed,
            track,
            date.format("%d%m%y")
        );

        let checksum = message.chars().skip(1).fold(0u8, |acc, c| acc ^ c as u8);

        format!("{message}*{checksum:X}\r\n")
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::Com;

    #[test]
    fn test_convert_gps_data_to_nmea_mid_gga() {
        let date = Utc.ymd(2022, 10, 30).and_hms_milli(21, 10, 30, 750);

        let data = crate::system::simconnect_objects::GpsData {
            lat: 51.509865,
            lon: -0.118092,
            alt: 0.0,
            pressure_altitude: 3.0,
            gps_ground_magnetic_track: 310.55,
            gps_magnetic_variation: 5.0,
            gps_ground_speed: 100.50,
        };

        let result = Com::convert_gps_data_to_nmea_mid_gga(&date, &data);

        assert_eq!(
            result,
            "$GPGGA,211030.750,5130.5919,N,00007.0855,W,1,12,1.0,3.0,M,0.0,M,,*70\r\n"
        );
    }

    #[test]
    fn test_convert_gps_data_to_nmea_mid_gga_2() {
        let date = Utc.ymd(2022, 1, 3).and_hms_milli(2, 1, 3, 75);

        let data = crate::system::simconnect_objects::GpsData {
            lat: 0.00040752447554520855,
            lon: 0.01397450300629543,
            alt: 0.0,
            pressure_altitude: 0.9642891859251844,
            gps_ground_magnetic_track: 92.71680515837362,
            gps_magnetic_variation: -4.384223296150313,
            gps_ground_speed: 0.0,
        };

        let result = Com::convert_gps_data_to_nmea_mid_gga(&date, &data);

        assert_eq!(
            result,
            "$GPGGA,020103.075,0000.0245,N,00000.8385,E,1,12,1.0,1.0,M,0.0,M,,*68\r\n"
        );
    }

    #[test]
    fn test_convert_gps_data_to_nmea_mid_rmc() {
        let date = Utc.ymd(2022, 10, 30).and_hms_milli(21, 10, 30, 750);

        let data = crate::system::simconnect_objects::GpsData {
            lat: 51.509865,
            lon: -0.118092,
            alt: 0.0,
            pressure_altitude: 3.0,
            gps_ground_magnetic_track: 310.55,
            gps_magnetic_variation: 5.0,
            gps_ground_speed: 100.50,
        };

        let result = Com::convert_gps_data_to_nmea_mid_rmc(&date, &data);

        assert_eq!(
            result,
            "$GPRMC,211030.750,A,5130.5919,N,00007.0855,W,100.5,305.6,301022,,,S*67\r\n"
        );
    }

    #[test]
    fn test_convert_gps_data_to_nmea_mid_rmc_2() {
        let date = Utc.ymd(2022, 1, 3).and_hms_milli(2, 1, 3, 75);

        let data = crate::system::simconnect_objects::GpsData {
            lat: 0.00040752447554520855,
            lon: 0.01397450300629543,
            alt: 0.0,
            pressure_altitude: 0.9642891859251844,
            gps_ground_magnetic_track: 92.71680515837362,
            gps_magnetic_variation: -4.384223296150313,
            gps_ground_speed: 0.0,
        };

        let result = Com::convert_gps_data_to_nmea_mid_rmc(&date, &data);

        assert_eq!(
            result,
            "$GPRMC,020103.075,A,0000.0245,N,00000.8385,E,0.0,97.1,030122,,,S*46\r\n"
        );
    }
}