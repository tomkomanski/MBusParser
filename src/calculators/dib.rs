use core::fmt;
use std::collections::VecDeque;
use serde::Serializer;
use serde_derive::Serialize;
use crate::matrices::errors::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
pub enum DibFunctionField {
    Instantaneous,
    Maximum,
    Minimum,
    ValueDuringError,
}

impl fmt::Display for DibFunctionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function_field: &str = match self {
            DibFunctionField::Instantaneous => "Instantaneous",
            DibFunctionField::Maximum => "Maximum",
            DibFunctionField::Minimum => "Minimum",
            DibFunctionField::ValueDuringError => "Value during error state",
        };
        write!(f, "{}", function_field)
    }
}

#[derive(Debug, PartialEq)]
pub enum DibDataType {
    NoData,
    Data8BitInteger,
    Data16BitInteger,
    Data24BitInteger,
    Data32BitInteger,
    Data32BitReal,
    Data48BitInteger,
    Data64BitInteger,
    SelectionForReadout,
    Data2DigitBCD,
    Data4DigitBCD,
    Data6DigitBCD,
    Data8DigitBCD,
    DataVariableLength,
    Data12DigitBCD,
    SpecialFunctionManufacturerSpecific,
    SpecialFunctionManufacturerSpecificExtandedNextDatagram,
    SpecialFunctionIdleFilter,
    SpecialFunctionReserved0x3F,
    SpecialFunctionReserved0x4F,
    SpecialFunctionReserved0x5F,
    SpecialFunctionReserved0x6F,
    SpecialFunctionGlobalReadout,
}

impl fmt::Display for DibDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_type: &str = match self {
            DibDataType::NoData => "No data",
            DibDataType::Data8BitInteger => "8 bit integer",
            DibDataType::Data16BitInteger => "16 bit integer",
            DibDataType::Data24BitInteger => "24 bit integer",
            DibDataType::Data32BitInteger => "32 bit integer",
            DibDataType::Data32BitReal => "32 bit real",
            DibDataType::Data48BitInteger => "48 bit integer",
            DibDataType::Data64BitInteger => "64 bit integer",
            DibDataType::SelectionForReadout => "Selection for readout",
            DibDataType::Data2DigitBCD => "2 digit BCD",
            DibDataType::Data4DigitBCD => "4 digit BCD",
            DibDataType::Data6DigitBCD => "6 digit BCD",
            DibDataType::Data8DigitBCD => "8 digit BCD",
            DibDataType::DataVariableLength => "Variable length",
            DibDataType::Data12DigitBCD => "12 digit BCD",
            DibDataType::SpecialFunctionManufacturerSpecific => "Manufacturer specific data",
            DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram => "Manufacturer specific data, more data in next datagram",
            DibDataType::SpecialFunctionIdleFilter => "Idle filter",
            DibDataType::SpecialFunctionReserved0x3F => "Reserved 0x3F",
            DibDataType::SpecialFunctionReserved0x4F => "Reserved 0x4F",
            DibDataType::SpecialFunctionReserved0x5F => "Reserved 0x5F",
            DibDataType::SpecialFunctionReserved0x6F => "Reserved 0x6F",
            DibDataType::SpecialFunctionGlobalReadout => "Global readout",
        };
        write!(f, "{}", data_type)
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Dib {
    #[serde(rename = "Dif byte", serialize_with = "serialize_dif_byte")]
    pub dif_byte: u8,
    #[serde(rename = "Dife bytes", serialize_with = "serialize_dife_bytes")]
    pub dife_bytes: Vec<u8>,
    #[serde(skip_serializing, skip_deserializing)]
    extension_bit: bool,
    #[serde(rename = "Storage number", serialize_with = "serialize_storage_tariff_subunit")]
    pub storage_number: Option<u32>,
    #[serde(rename = "Tariff", serialize_with = "serialize_storage_tariff_subunit")]
    pub tariff: Option<u32>,
    #[serde(rename = "Subunit", serialize_with = "serialize_storage_tariff_subunit")]
    pub subunit: Option<u32>,
    #[serde(rename = "Function field", serialize_with = "serialize_function_field")]
    pub function_field: Option<DibFunctionField>,
    #[serde(rename = "Data type", serialize_with = "serialize_data_type")]
    pub data_type: DibDataType,
    #[serde(skip_serializing)]
    pub data_length: u8,
}

fn serialize_dif_byte<S>(x: &u8, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            let string: String = byte_to_hex_string(x);
            s.serialize_str(&string)
        }

fn serialize_dife_bytes<S>(x: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
            if x.is_empty() {
                s.serialize_none()
            }
            else {          
                let hex_string: String = array_bytes_to_hex_string(&x);
                s.serialize_str(&hex_string)
            }
        }
    
fn serialize_storage_tariff_subunit<S>(x: &Option<u32>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
            match x {
                Some(x) => {
                    s.serialize_u32(*x)
                },
                None => {
                    s.serialize_none()
                }
            }
        }

fn serialize_function_field<S>(x: &Option<DibFunctionField>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            match x {
                Some(x) => {
                    s.serialize_str(&x.to_string())
                },
                None => {
                    s.serialize_none()
                }
            }
        }

fn serialize_data_type<S>(x: &DibDataType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            let string: String =  x.to_string();
            s.serialize_str(&string)
        }

impl Dib {
    pub fn calculate_dib(data: &mut VecDeque<u8>) -> Result<Dib, ParserError> {
        if data.len() < 1 {
            return Err(ParserError::DibCalculatorError);
        }

        let dif_byte: u8 = data.pop_front().unwrap();
        let data_type: DibDataType = DibDataType::get_data_type(dif_byte);
        let data_length: u8 = DibDataType::get_data_length(&data_type);

        let mut dib: Dib = Dib {
            dif_byte: dif_byte,
            dife_bytes: vec![],
            extension_bit: false,
            storage_number: None,
            tariff: None,
            subunit: None,
            function_field: None,
            data_type: data_type,
            data_length: data_length,
        };

        if dib.data_type == DibDataType::SpecialFunctionIdleFilter || 
            dib.data_type == DibDataType::SpecialFunctionManufacturerSpecific || 
            dib.data_type == DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram ||
            dib.data_type == DibDataType::SpecialFunctionReserved0x3F || 
            dib.data_type == DibDataType::SpecialFunctionReserved0x4F || 
            dib.data_type == DibDataType::SpecialFunctionReserved0x5F || 
            dib.data_type == DibDataType::SpecialFunctionReserved0x6F ||
            dib.data_type == DibDataType::SpecialFunctionGlobalReadout {

            return Ok(dib);
        }

        dib.function_field = Some(DibFunctionField::get_function(dib.dif_byte));
        dib.extension_bit = (dib.dif_byte & 0x80) >> 7 == 1;
        dib.storage_number = Some(((dib.dif_byte & 0x40) >> 6) as u32);

        let mut storage_number: u32 = dib.storage_number.unwrap();
        let mut subunit: u32 = 0;
        let mut tariff: u32 = 0;
        let mut loop_count: u8 = 0;

        while dib.extension_bit {
            if data.len() < 1 {
                return Err(ParserError::DibCalculatorError);
            }

            let dife_byte: u8 = data.pop_front().unwrap();

            dib.dife_bytes.push(dife_byte);
            dib.extension_bit = (dife_byte & 0x80) >> 7 == 1;

            let subunit_loop: u32 = ((dife_byte & 0x40) >> 6) as u32;
            let tariff_loop: u32 = ((dife_byte & 0x30) >> 4) as u32;
            let storage_number_loop: u32 = (dife_byte & 0x0f) as u32;

            subunit |= subunit_loop << loop_count;
            tariff |= tariff_loop << (loop_count * 2);
            storage_number |= storage_number_loop << (loop_count * 4 + 1);
            
            loop_count += 1;
        }

        if loop_count != 0 {
            dib.storage_number = Some(storage_number);
            dib.subunit = Some(subunit);
            dib.tariff = Some(tariff);
        }

        return Ok(dib);
    }
}

impl DibFunctionField {
    fn get_function(byte: u8) -> DibFunctionField {
        let function_field: u8 = (byte & 0x30) >> 4;
        match function_field {
            0x00 => DibFunctionField::Instantaneous,
            0x01 => DibFunctionField::Maximum,
            0x02 => DibFunctionField::Minimum,
            _ => DibFunctionField::ValueDuringError,
        }
    }
}

impl DibDataType {
    pub fn get_data_type(byte: u8) -> DibDataType {
        if byte & 0x0F == 0x0F {
            match byte {
                0x0F => DibDataType::SpecialFunctionManufacturerSpecific,
                0x1F => DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram,
                0x2F => DibDataType::SpecialFunctionIdleFilter,
                0x3F => DibDataType::SpecialFunctionReserved0x3F,
                0x4F => DibDataType::SpecialFunctionReserved0x4F,
                0x5F => DibDataType::SpecialFunctionReserved0x5F,
                0x6F => DibDataType::SpecialFunctionReserved0x6F,
                _ => DibDataType::SpecialFunctionGlobalReadout,
            }
        }
        else {
            let data_type: u8 = byte & 0x0F;
            match data_type {
                0x00 => DibDataType::NoData,
                0x01 => DibDataType::Data8BitInteger,
                0x02 => DibDataType::Data16BitInteger,
                0x03 => DibDataType::Data24BitInteger,
                0x04 => DibDataType::Data32BitInteger,
                0x05 => DibDataType::Data32BitReal,
                0x06 => DibDataType::Data48BitInteger,
                0x07 => DibDataType::Data64BitInteger,
                0x08 => DibDataType::SelectionForReadout,
                0x09 => DibDataType::Data2DigitBCD,
                0x0A => DibDataType::Data4DigitBCD,
                0x0B => DibDataType::Data6DigitBCD,
                0x0C => DibDataType::Data8DigitBCD,
                0x0D => DibDataType::DataVariableLength,
                _ => DibDataType::Data12DigitBCD,
            }
        }
    }

    pub fn get_data_length(data_type: &DibDataType) -> u8 {
        let length_in_byte: u8 = match data_type {
            DibDataType::NoData => 0,
            DibDataType::Data8BitInteger => 1,
            DibDataType::Data16BitInteger => 2,
            DibDataType::Data24BitInteger => 3,
            DibDataType::Data32BitInteger => 4,
            DibDataType::Data32BitReal => 4,
            DibDataType::Data48BitInteger => 6,
            DibDataType::Data64BitInteger => 8,
            DibDataType::SelectionForReadout => 0,
            DibDataType::Data2DigitBCD => 1,
            DibDataType::Data4DigitBCD => 2,
            DibDataType::Data6DigitBCD => 3,
            DibDataType::Data8DigitBCD => 4,
            DibDataType::DataVariableLength => 1,
            DibDataType::Data12DigitBCD => 6,
            DibDataType::SpecialFunctionManufacturerSpecific => 0,
            DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram => 0,
            DibDataType::SpecialFunctionIdleFilter => 0,
            DibDataType::SpecialFunctionReserved0x3F => 0,
            DibDataType::SpecialFunctionReserved0x4F => 0,
            DibDataType::SpecialFunctionReserved0x5F => 0,
            DibDataType::SpecialFunctionReserved0x6F => 0,
            DibDataType::SpecialFunctionGlobalReadout => 0,
        };
        return length_in_byte;
    }

    pub fn calculate_data(data_type: &DibDataType, bytes: &[u8]) -> Result<Option<f64>, ParserError> {
        match data_type {
            DibDataType::NoData => {
                if bytes.len() != 0 {
                    return Err(ParserError::DibCalculatorError);
                }

                return Ok(None);
            }
            DibDataType::Data8BitInteger => {
                if bytes.len() != 1 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = i8::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            }
            DibDataType::Data16BitInteger => {
                if bytes.len() != 2 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 2], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = i16::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data24BitInteger => {
                if bytes.len() != 3 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 3], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_24_to_int_32(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data32BitInteger => {
                if bytes.len() != 4 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = i32::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data32BitReal => {
                if bytes.len() != 4 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = f32::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data48BitInteger => {
                if bytes.len() != 6 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 6], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_48_to_int_64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data64BitInteger => {
                if bytes.len() != 8 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 8], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = i64::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::SelectionForReadout => {
                if bytes.len() != 0 {
                    return Err(ParserError::DibCalculatorError);
                }

                return Ok(None);
            },
            DibDataType::Data2DigitBCD => {
                if bytes.len() != 1 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data4DigitBCD => {
                if bytes.len() != 2 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 2], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data6DigitBCD => {
                if bytes.len() != 3 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 3], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data8DigitBCD => {
                if bytes.len() != 4 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::DataVariableLength => {
                if bytes.len() != 1 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = u8::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data12DigitBCD => {
                if bytes.len() != 6 {
                    return Err(ParserError::DibCalculatorError);
                }

                let bytes: Result<[u8; 6], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DibCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::SpecialFunctionManufacturerSpecific => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionIdleFilter => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionReserved0x3F => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionReserved0x4F => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionReserved0x5F => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionReserved0x6F => {
                return Ok(None);
            },
            DibDataType::SpecialFunctionGlobalReadout => {
                return Ok(None);
            },
        };
    }
}

#[cfg(test)]
#[path = "tests/tests_dib.rs"]
mod tests_dib_calculator;