use std::ffi::c_void;
use std::{convert::TryFrom, fmt::Debug};

mod bindings;

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
        std::ffi::CString::new($target).unwrap().as_ptr();
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
        define_id: u32,
        datum_name: &str,
        units_name: &str,
    ) -> Result<(), i32> {
        unsafe {
            success!(bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                define_id,
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
        define_id: u32,
        object_id: u32,
        period: PeriodEnum,
    ) -> Result<(), i32> {
        unsafe {
            let simconnect_period = match period {
                PeriodEnum::VisualFrame => {
                    bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME
                }
                PeriodEnum::Second => bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SECOND,
            };

            success!(bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                define_id,
                object_id,
                simconnect_period,
                0,
                0,
                0,
                0,
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
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => unsafe {
                let event: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA = std::mem::transmute_copy(
                    &(data_buf as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA),
                );

                let data: &bindings::DWORD = &event.dwData;

                Some(Notification::Data(event.dwDefineID, data))
            },
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => None,
            _ => panic!("Got unrecognized notification: {}", unsafe {
                (*data_buf).dwID as i32
            }),
        };

        Ok(result)
    }
}

pub enum Notification<'a> {
    Open,
    Event(Event),
    Data(u32, &'a u32),
    Quit,
}

#[derive(Debug, Copy, Clone, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum Event {
    Brakes,
    BrakesLeft,
    AxisLeftBrakeSet,
}

#[derive(Debug)]
pub enum PeriodEnum {
    VisualFrame,
    Second,
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
