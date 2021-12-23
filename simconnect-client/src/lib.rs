use helpers::fixed_c_str_to_string;
use std::ffi::c_void;
use std::{convert::TryFrom, fmt::Debug};

mod bindings;
mod helpers;

macro_rules! success {
    ($hr:expr) => {{
        let hr = $hr;
        if hr != 0 {
            return Err(hr);
        }
    }};
}

macro_rules! ok_if_fail {
    ($hr:expr, $ret:expr) => {{
        let hr = $hr;
        let ret = $ret;
        if hr != 0 {
            return Ok(ret);
        }
    }};
}

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target)
            .expect("failed to create CString")
            .as_ptr()
    };
}

#[derive(Debug)]
pub struct SimConnect {
    pub handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    #[tracing::instrument(name = "SimConnect::new")]
    pub fn new(name: &str) -> Result<Self, bindings::HRESULT> {
        let mut handle = std::ptr::null_mut();

        success!(unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                as_c_string!(name),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        });

        Ok(Self {
            handle: std::ptr::NonNull::new(handle)
                .expect("ERROR: SimConnect_Open returned null pointer on success"),
        })
    }

    #[tracing::instrument(name = "SimConnect::register_event")]
    pub fn register_event(&self, event: Event) -> Result<(), i32> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        });

        let group = Group::Group0;

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        });

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(self.handle.as_ptr(), group as u32, 1)
        });

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::add_to_data_definition")]
    pub fn add_to_data_definition(
        &self,
        request_id: u32,
        datum_name: &str,
        units_name: &str,
    ) -> Result<(), i32> {
        unsafe {
            success!(bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                request_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                u32::MAX,
            ));
        }

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::request_data_on_sim_object")]
    pub fn request_data_on_sim_object(
        &self,
        request_id: u32,
        period: PeriodEnum,
        condition: ConditionEnum,
    ) -> Result<(), i32> {
        unsafe {
            let (simconnect_period, simconnect_interval) = match period {
                PeriodEnum::VisualFrame { interval } => (
                    bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME,
                    interval,
                ),
                PeriodEnum::Second => (bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SECOND, 0),
            };

            let simconnect_flags: u32 = match condition {
                ConditionEnum::None => 0,
                ConditionEnum::Changed => bindings::SIMCONNECT_DATA_REQUEST_FLAG_CHANGED,
            };

            success!(bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                request_id,
                request_id,
                simconnect_period,
                simconnect_flags,
                0,
                simconnect_interval,
                0,
            ));
        }

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::subscribe_to_airport_list")]
    pub fn subscribe_to_airport_list(&self, request_id: u32) -> Result<(), i32> {
        unsafe {
            success!(bindings::SimConnect_SubscribeToFacilities(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT,
                request_id,
            ));
        }

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::request_airport_list")]
    pub fn request_airport_list(&self, request_id: u32) -> Result<(), i32> {
        unsafe {
            success!(bindings::SimConnect_RequestFacilitiesList(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT,
                request_id,
            ));
        }

        Ok(())
    }

    pub fn get_next_dispatch(&self) -> Result<Option<Notification>, i32> {
        let mut data_buf: *mut bindings::SIMCONNECT_RECV = std::ptr::null_mut();
        let mut size_buf: bindings::DWORD = 32;
        let size_buf_pointer: *mut bindings::DWORD = &mut size_buf;

        unsafe {
            ok_if_fail!(
                bindings::SimConnect_GetNextDispatch(
                    self.handle.as_ptr(),
                    &mut data_buf,
                    size_buf_pointer
                ),
                None
            );
        };

        let result = match unsafe { (*data_buf).dwID as i32 } {
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Some(Notification::Open),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Some(Notification::Quit),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                let event = unsafe { *(data_buf as *const bindings::SIMCONNECT_RECV_EVENT) };
                let event = Event::try_from(event.uEventID).expect("Unrecognized event");
                Some(Notification::Event(event))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                let event: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA),
                    )
                };

                let data_addr = std::ptr::addr_of!(event.dwData);

                Some(Notification::Data(event.dwDefineID, data_addr))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                let event: &bindings::SIMCONNECT_RECV_AIRPORT_LIST = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_AIRPORT_LIST),
                    )
                };

                let data = event
                    .rgData
                    .iter()
                    .map(|data| AirportData {
                        icao: fixed_c_str_to_string(&data.Icao),
                        lat: data.Latitude,
                        lon: data.Longitude,
                        alt: data.Altitude,
                    })
                    .collect::<Vec<_>>();

                Some(Notification::AirportList(data))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                let event = unsafe { *(data_buf as *const bindings::SIMCONNECT_RECV_EXCEPTION) };
                Some(Notification::Exception(event.dwException))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => None,
            _ => panic!("Got unrecognized notification: {}", unsafe {
                (*data_buf).dwID as i32
            }),
        };

        Ok(result)
    }
}

pub enum Notification {
    Open,
    Event(Event),
    Data(u32, *const u32),
    AirportList(Vec<AirportData>),
    Quit,
    Exception(u32),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AirportData {
    icao: String,
    lat: f64,
    lon: f64,
    alt: f64,
}

#[derive(Debug, Copy, Clone, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum Event {
    Brakes,
    BrakesLeft,
    AxisLeftBrakeSet,
}

#[derive(Debug, Clone)]
pub enum PeriodEnum {
    VisualFrame { interval: u32 },
    Second,
}

#[derive(Debug, Clone)]
pub enum ConditionEnum {
    None,
    Changed,
}

use std::os::raw::c_char;
impl Event {
    fn into_c_char(self) -> *const c_char {
        match self {
            Event::Brakes => "BRAKES\0".as_ptr() as *const c_char,
            Event::BrakesLeft => "BRAKES_LEFT\0".as_ptr() as *const c_char,
            Event::AxisLeftBrakeSet => "AXIS_LEFT_BRAKE_SET\0".as_ptr() as *const c_char,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum Group {
    Group0,
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
