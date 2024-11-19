use std::collections::VecDeque;
use crate::calculators::vib::*;

pub fn get_manufacturer_specific_vife_after_primary_vif (data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
    if data.len() < 1 {
        return false;
    }
    let vife_byte: u8 = data.pop_front().unwrap();

    let vife: VifVife = VifVife::get_vife_schneider(vife_byte);

    vib.vife_bytes.push(vife_byte);
    vib.extension = vife.extension;    
    vib.data_type = vife.data_type;
    vib.unit = vife.unit.to_string();
    vib.description = vife.description.to_string();
    vib.magnitude = vife.magnitude;

    while vib.extension.is_some() {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();

        let mut extension: Option<u8> = None;
        if ((vife_byte & 0x80) >> 7) == 1 {
            extension = Some(vife_byte);
        }

        vib.vife_bytes.push(vife_byte);
        vib.extension = extension;
    }

    return true;
}

pub fn get_manufacturer_specific_vife_after_combinable_vife(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
    if data.len() < 1 {
        return false;
    }
    let vife_byte: u8 = data.pop_front().unwrap();

    let vife: VifVife = VifVife::get_vife_schneider(vife_byte);

    vib.vife_bytes.push(vife_byte);
    vib.extension = vife.extension; 
    vib.description = vib.description.to_string() + " " + &vife.description;

    while vib.extension.is_some() {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();

        let mut extension: Option<u8> = None;
        if ((vife_byte & 0x80) >> 7) == 1 {
            extension = Some(vife_byte);
        }

        vib.vife_bytes.push(vife_byte);
        vib.extension = extension;
    }

    return true;
}