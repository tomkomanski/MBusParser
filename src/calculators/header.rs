use std::collections::VecDeque;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::matrices::errors::*;
use crate::matrices::encryption_method::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub header_type: HeaderType,
    pub identification_number: Option<[u8; 4]>,
    pub manufacturer: Option<[u8; 2]>,
    pub version: Option<u8>,
    pub device_type: Option<u8>,
    pub device_type_hum: Option<String>,
    pub access_number: Option<u8>,
    pub status: Option<u8>,
    pub configuration: Option<[u8; 2]>,
    pub encryption: Option<EncryptionMethod>,
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Header", 9)?;

        match &self.identification_number {
            Some(x) => {
                state.serialize_field("Identification number", &bcd_to_u64(x))?;
            },
            None => {
                state.serialize_field("Identification number", &Option::<[u8; 4]>::None)?;
            }
        }

        state.serialize_field("Manufacturer", &self.get_manufacturer())?;
        state.serialize_field("Version", &self.version)?;
        state.serialize_field("Device type", &self.device_type)?;
        state.serialize_field("Device type hum", &self.device_type_hum)?;
        state.serialize_field("Access number", &self.access_number)?;
        state.serialize_field("Status", &self.get_status_description())?;

        match &self.configuration {
            Some(x) => {
                state.serialize_field("Configuration", &array_bytes_to_hex_string(x))?;
            },
            None => {
                state.serialize_field("Configuration", &Option::<[u8; 2]>::None)?;
            }
        }

        match &self.encryption {
            Some(x) => {
                state.serialize_field("Encryption", &x.to_string())?;
            },
            None => {
                state.serialize_field("Encryption", &Option::<String>::None)?;
            }
        }

        state.end()
    }
}

#[derive(Debug, PartialEq)]
pub enum HeaderType {
    None,
    Short,
    Long,
    Unknown,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            header_type: HeaderType::Unknown,
            identification_number: None,
            manufacturer: None,
            version: None,
            device_type: None,
            device_type_hum: None,
            access_number: None,
            status: None,
            configuration: None,
            encryption: None,
        }
    }
}

impl HeaderType {
    fn new(ci_field: u8) -> HeaderType {  
        match ci_field {
            0x50 | 0x51 | 0x52 | 0x69 | 0x70 | 0x71 | 0x78 | 0x79 => HeaderType::None,
            0x5A | 0x61 | 0x65 | 0x6A | 0x6E | 0x74 | 0x7A | 0x7B | 0x7D | 0x7F | 0x8A => HeaderType::Short,
            0x53 | 0x5B | 0x60 | 0x64 | 0x6B | 0x6C | 0x6D | 0x6F | 0x72 | 0x73 | 0x75 | 0x7C | 0x7E | 0x80 | 0x84 | 0x85 | 0x8B => HeaderType::Long,    
            _ => HeaderType::Unknown
        }
    }
}

impl Header {
    pub fn new(ci_field: u8, data: &mut VecDeque<u8>) -> Result<Self, ParserError> {  
        let mut header: Header = Header {..Header::default() };
        header.header_type = HeaderType::new(ci_field);
        
        if data.len() < header.get_length() as usize {
            return Err(ParserError::HeaderCalculatorError);
        }

        if header.header_type == HeaderType::None {
            return Ok(header);
        }
        else if header.header_type == HeaderType::Short {
            header.access_number = data.pop_front();
            header.status = data.pop_front();
            header.configuration = Some([data.pop_front().unwrap(), *data.front().unwrap()]);
            header.encryption = Some(EncryptionMethod::new(data.pop_front().unwrap()));
            return Ok(header);
        }
        else if header.header_type == HeaderType::Long {
            header.identification_number = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
            header.manufacturer = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
            header.version = data.pop_front();
            header.device_type = data.pop_front();
            header.access_number = data.pop_front();
            header.status = data.pop_front();
            header.configuration = Some([data.pop_front().unwrap(), *data.front().unwrap()]);
            header.encryption = Some(EncryptionMethod::new(data.pop_front().unwrap()));
            return Ok(header);
        }
        else {
            return Err(ParserError::HeaderCalculatorError);
        }
    }

    fn get_manufacturer(&self) -> Option<String> {
        if let Some(bytes) = self.manufacturer {
            let input: u16 = u16::from_le_bytes(bytes.clone());
            let first_letter: char = ((input / (32 * 32)) + 64) as u8 as char;
            let second_letter: char = (((input % (32 * 32)) / 32) + 64) as u8 as char;
            let third_letter: char = ((input % 32) + 64) as u8 as char;

            if !first_letter.is_ascii() || !second_letter.is_ascii() || !third_letter.is_ascii() {
                return None;
            }

            let mut manufacturer: String = String::new();
            manufacturer.push(first_letter);
            manufacturer.push(second_letter);
            manufacturer.push(third_letter);

            return Some(manufacturer);
        }
        else {
            return None;
        }
    }

    fn get_status_description(&self) -> Option<String> {
        if let Some(byte) = self.status {
            let mut status: String = String::new();
            let bit0: bool = ((byte >> 0) & 0x01) == 1;
            let bit1: bool = ((byte >> 1) & 0x01) == 1;
            let bit2: bool = ((byte >> 2) & 0x01) == 1;
            let bit3: bool = ((byte >> 3) & 0x01) == 1;
            let bit4: bool = ((byte >> 4) & 0x01) == 1;
            let bit5: bool = ((byte >> 5) & 0x01) == 1;
            let bit6: bool = ((byte >> 6) & 0x01) == 1;
            let bit7: bool = ((byte >> 7) & 0x01) == 1;

            if !bit1 && !bit0 {
                status.push_str("[No error] ");
            }
            if !bit1 && bit0 {
                status.push_str("[Application busy] ");
            }
            if bit1 && !bit0 {
                status.push_str("[Any application error] ");
            }
            if bit1 && bit0 {
                status.push_str("[Abnormal condition / alarm] ");
            }
            if bit2 {
                status.push_str("[Power low] ");
            }
            if bit3 {
                status.push_str("[Permanent error] ");
            }
            if bit4 {
                status.push_str("[Temporary error] ");
            }
            if bit5 {
                status.push_str("[Specific to manufacturer bit 5] ");
            }
            if bit6 {
                status.push_str("[Specific to manufacturer bit 6] ");
            }
            if bit7 {
                status.push_str("[Specific to manufacturer bit 7] ");
            }

            return Some(status.trim().to_string());
        }

        return None;
    }

    pub fn get_length(&self) -> u8 {
        match self.header_type {
            HeaderType::None => 0,
            HeaderType::Short => 4,
            HeaderType::Long => 12,
            HeaderType::Unknown => 0
        }
    }
}

#[cfg(test)]
#[path = "tests/tests_header.rs"]
mod tests_header_calculator;