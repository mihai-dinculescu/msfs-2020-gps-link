use std::ffi::CString;

pub fn fixed_c_str_to_string(data: &[i8]) -> String {
    let u8slice = unsafe { &*(data as *const _ as *const [u8]) };

    let mut value = u8slice.to_vec();

    let pos = value.iter().position(|c| *c == 0).unwrap_or(value.len());

    value.truncate(pos);
    let icao = unsafe { CString::from_vec_unchecked(value) };

    icao.to_str().unwrap().to_string()
}
