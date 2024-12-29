mod calculators {
    mod manufacturer_specific {
        pub mod abb;
        pub mod schneider;
        pub mod unknown;
    }
    pub mod compact_profile;
    pub mod data_record;
    pub mod dib;
    pub mod extended_link_layer;
    pub mod header;
    pub mod lvar;
    pub mod telegram_format;
    pub mod vib;
}

mod matrices {
    pub mod manufacturer_specific {
        pub mod manufacturer;
        pub mod abb_vife;
        pub mod abb_vife_f8;
        pub mod abb_vife_f9;
        pub mod abb_vife_fe;
        pub mod schneider_vife;
    }
    pub mod errors;
    pub mod device_type;
    pub mod encryption_method;
    pub mod result_models;
    pub mod vif_primary;
    pub mod vife_combinable;
    pub mod vife_combinable_fc;
    pub mod vife_ef;
    pub mod vife_fb;
    pub mod vife_fd;
    pub mod vife_ff;
}

pub mod parser;

mod frame_parsers {
    pub mod long_frame_mbus;
    pub mod long_frame_wmbus_format_a;
    pub mod long_frame_wmbus_format_b;

}

mod post_processing {
    pub mod post_processing;
    pub mod ngp_postprocess;
    pub mod wireless_mbus_data_conteiner;
}

mod tools {
    pub mod checksum;
    pub mod tools;   
}

use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern fn parse(frame_input: *const c_char, key_input: *const c_char) -> *mut c_char {
    let mut frame: &str = "";
    let mut key: &str = "";

    if !frame_input.is_null() {
        let c_frame: &CStr = unsafe { CStr::from_ptr(frame_input) };
        let frame_result: Result<&str, std::str::Utf8Error> = c_frame.to_str();
        if frame_result.is_ok() {
            frame = frame_result.unwrap();
        }
        else {
            let output: String = String::from("M-Bus parser library fatal error: incorrect frame data");
            let pntr: *mut i8 = CString::new(output).unwrap().into_raw();
            return pntr;
        }
    }
    
    if !key_input.is_null() {
        let c_key: &CStr = unsafe { CStr::from_ptr(key_input) };
        let key_result: Result<&str, std::str::Utf8Error> = c_key.to_str();
        if key_result.is_ok() {
            key = key_result.unwrap();
        }
        else {
            let output: String = String::from("M-Bus parser library fatal error: incorrect key data");
            let pntr: *mut i8 = CString::new(output).unwrap().into_raw();
            return pntr;
        }
    }

    let output: String = parser::parse_telegram(frame, key);    

    let pntr: *mut i8 = CString::new(output).unwrap().into_raw();
    return pntr;
}

#[no_mangle]
pub extern fn release_resource(sp: *mut c_char) {
    unsafe {
        if sp.is_null() {
            return;
        }
        
        let _ = CString::from_raw(sp);
    }
}