use std::collections::VecDeque;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::calculators::manufacturer_specific::*;
use crate::matrices::manufacturer_specific::manufacturer::*;
use crate::matrices::errors::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
pub enum VibDataType {
    AnyVif,
    CompactProfileWithRegister,
    CompactProfile,
    CustomString,
    DataTypeFJIM,
    DataTypeG,
    ManufacturerSpecificDataContainer,
    Numeric,
    StandardConformData,
    WirelessMbusDataContainer,
}

#[derive(Debug, PartialEq)]
pub struct VifVife<'a> {
    pub byte: u8,
    pub extension: Option<u8>,  
    pub data_type: Option<VibDataType>, 
    pub unit: &'a str,
    pub description: &'a str,
    pub magnitude: Option<i8>, 
}

#[derive(Debug, PartialEq)]
pub struct Vib {
    pub vif_byte: u8,
    pub vife_bytes: Vec<u8>,
    pub extension: Option<u8>,
    pub data_type: Option<VibDataType>,
    pub unit: String,
    pub description: String,
    pub magnitude: Option<i8>,
}

impl Serialize for Vib {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Vib", 4)?;

        state.serialize_field("Vif byte", &byte_to_hex_string(&self.vif_byte))?;

        if self.vife_bytes.is_empty() {
            state.serialize_field("Vife bytes", &Option::<String>::None)?;
        }
        else {
            state.serialize_field("Vife bytes", &array_bytes_to_hex_string(&self.vife_bytes))?;
        }

        if self.unit.is_empty() {
            state.serialize_field("Unit", &Option::<String>::None)?;
        }
        else {
            state.serialize_field("Unit", &self.unit)?;
        }

        if self.description.is_empty() {
            state.serialize_field("Description", &Option::<String>::None)?;
        }
        else {
            state.serialize_field("Description", &self.description)?;
        }

        state.end()
    }
}

impl Vib {
    pub fn new(data: &mut VecDeque<u8>, manufacturer: &Option<[u8; 2]>) -> Result<Self, ParserError> {
        if data.len() < 1 {
            return Err(ParserError::VibCalculatorError);
        }

        let specific_manufacturer: Manufacturer = Manufacturer::get_manufacturer(manufacturer);

        let primary_vif: VifVife = VifVife::get_vif_primary(data.pop_front().unwrap());

        let mut vib: Vib = Vib {
            vif_byte: primary_vif.byte,
            vife_bytes: vec![],
            extension: primary_vif.extension,
            data_type: primary_vif.data_type,
            unit: primary_vif.unit.to_string(),
            description: primary_vif.description.to_string(),
            magnitude: primary_vif.magnitude,
        };

        // Manufacturer specific block VIFE 0x7F after primary VIF
        if vib.extension.is_some_and(|x: u8| x == 0x7F) {
            return Ok(vib);
        }
        // Manufacturer specific block VIFE 0xFF after primary VIF
        else if vib.extension.is_some_and(|x: u8| x == 0xFF)  {
            let result: bool = match specific_manufacturer {
                Manufacturer::Abb => abb::get_manufacturer_specific_vife_after_primary_vif(data, &mut vib),
                Manufacturer::Schneider => schneider::get_manufacturer_specific_vife_after_primary_vif(data, &mut vib),
                Manufacturer::Unknown => unknown::get_manufacturer_specific_vife_after_primary_vif(data, &mut vib),
            };
            if !result {
                return Err(ParserError::VibCalculatorError);
            }
            return Ok(vib);
        }
        
        // VIFE extension table 0xEF after primary VIF
        else if vib.extension.is_some_and(|x: u8| x == 0xEF) {
            if !Self::get_extension_ef_after_primary_vif(data, &mut vib) {
                return Err(ParserError::VibCalculatorError);
            }
        }
        // VIFE extension table 0xFB after primary VIF
        else if vib.extension.is_some_and(|x: u8| x == 0xFB) {
            if !Self::get_extension_fb_after_primary_vif(data, &mut vib) {
                return Err(ParserError::VibCalculatorError);
            }
        }
        // VIFE extension table 0xFD after primary VIF
        else if vib.extension.is_some_and(|x: u8| x == 0xFD) {
            if !Self::get_extension_fd_after_primary_vif(data, &mut vib) {
                return Err(ParserError::VibCalculatorError);
            }
        }

        // VIFE combinable
        if vib.extension.is_some() {
            if !Self::get_combinable_vife(data, &mut vib) {
                return Err(ParserError::VibCalculatorError);
            }

            while vib.extension.is_some_and(|x: u8| x != 0x7F && x != 0xFF && x != 0xFC) {
                if !Self::get_combinable_vife(data, &mut vib) {
                    return Err(ParserError::VibCalculatorError);
                }
            }
        }

        // Manufacturer specific block VIFE 0x7F after VIFE combinable
        if vib.extension.is_some_and(|x: u8| x == 0x7F) {
            return Ok(vib);
        }
        // Manufacturer specific block VIFE 0xFF after VIFE combinable
        else if vib.extension.is_some_and(|x: u8| x == 0xFF)  {
            let result: bool =  match specific_manufacturer {
                Manufacturer::Abb => abb::get_manufacturer_specific_vife_after_combinable_vife(data, &mut vib),
                Manufacturer::Schneider => schneider::get_manufacturer_specific_vife_after_combinable_vife(data, &mut vib),
                Manufacturer::Unknown => unknown::get_manufacturer_specific_vife_after_combinable_vife(data, &mut vib),
            };
            if !result {
                return Err(ParserError::VibCalculatorError);
            }
            return Ok(vib);
        }

        // VIFE combinable extension table 0xFC after VIFE combinable 
        if vib.extension.is_some_and(|x: u8| x == 0xFC) {
            if !Self::get_combinable_vife_extension_fc(data, &mut vib) {
                return Err(ParserError::VibCalculatorError);
            }
        }

        // Vif custom string
        if (vib.vif_byte == 0xFC || vib.vif_byte == 0x7C) && vib.extension.is_none() {
            if data.len() < 1 {
                return Err(ParserError::VibCalculatorError);
            }
            let string_length: u8 = data.pop_front().unwrap();

            vib.vife_bytes.push(string_length);
            if data.len() < string_length as usize {
                return Err(ParserError::VibCalculatorError);
            }
            let text_bytes: Vec<u8> = data.drain(..string_length as usize).collect();
            
            vib.vife_bytes.extend(&text_bytes);
            let text_bytes_reversed: Vec<u8> = text_bytes.iter().copied().rev().collect();
            let text_result: Result<String, std::string::FromUtf8Error> = String::from_utf8(text_bytes_reversed);
            if text_result.is_err() {
                return Err(ParserError::VibCalculatorError);
            }
            vib.data_type = Some(VibDataType::Numeric);
            vib.description = (text_result.unwrap().to_string() + " " + &vib.description).trim().to_string();
        }
        
        return Ok(vib);
    }

    fn get_extension_ef_after_primary_vif(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();
        
        let vife: VifVife = VifVife::get_vife_ef(vife_byte);

        vib.vife_bytes.push(vife_byte);
        vib.extension = vife.extension;   
        vib.data_type = vife.data_type;
        vib.unit = vife.unit.to_string();
        vib.description = vife.description.to_string();
        vib.magnitude = vife.magnitude;

        return true;
    }

    fn get_extension_fb_after_primary_vif(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();
        
        let vife: VifVife = VifVife::get_vife_fb(vife_byte);

        vib.vife_bytes.push(vife_byte);
        vib.extension = vife.extension;   
        vib.data_type = vife.data_type;
        vib.unit = vife.unit.to_string();
        vib.description = vife.description.to_string();
        vib.magnitude = vife.magnitude;

        return true;
    }

    fn get_extension_fd_after_primary_vif(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();
        
        let vife: VifVife = VifVife::get_vife_fd(vife_byte);

        vib.vife_bytes.push(vife_byte);
        vib.extension = vife.extension;     
        vib.data_type = vife.data_type;
        vib.unit = vife.unit.to_string();
        vib.description = vife.description.to_string();
        vib.magnitude = vife.magnitude;

        return true;
    }

    fn get_combinable_vife(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();

        let vife: VifVife = VifVife::get_vife_combinable(vife_byte);

        vib.vife_bytes.push(vife_byte);
        vib.extension = vife.extension;

        if vib.extension.is_some_and(|x: u8| x == 0x7F || x == 0xFF || x == 0xFC) {
            return true;
        }

        vib.description = (vib.description.to_string() + " " + vife.description).trim().to_string();

        if vife.magnitude.is_some() {
            vib.magnitude = vife.magnitude;
        }

        if vife.data_type.is_some() {
            vib.data_type = vife.data_type;
        }
        
        return true;
    }

    fn get_combinable_vife_extension_fc(data: &mut VecDeque<u8>, vib: &mut Vib) -> bool {
        if data.len() < 1 {
            return false;
        }
        let vife_byte: u8 = data.pop_front().unwrap();
        
        let vife: VifVife = VifVife::get_vife_combinable_fc(vife_byte);

        vib.vife_bytes.push(vife_byte);
        vib.extension = vife.extension; 
        vib.description = (vib.description.to_string() + " " + vife.description).trim().to_string();

        return true;
    }

    pub fn update_vib(&mut self, unit: Option<&str>, description: Option<&str>) {
        if unit.is_some() {
            self.unit = unit.unwrap().to_string();
        }
        if description.is_some() {
            self.description = description.unwrap().to_string();
        }
    }
}

#[cfg(test)]
#[path = "tests/tests_vib.rs"]
mod tests_vib_calculator;