use core::fmt;
use std::collections::VecDeque;
use serde::ser::{Serialize, Serializer, SerializeStruct};
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

#[derive(Debug, PartialEq)]
pub struct Dib {
    pub dif_byte: u8,
    pub dife_bytes: Vec<u8>,
    extension_bit: bool,
    pub storage_number: Option<u32>,
    pub tariff: Option<u32>,
    pub subunit: Option<u32>,
    pub function_field: Option<DibFunctionField>,
    pub data_type: DibDataType,
    pub data_length: u8,
}

impl Serialize for Dib {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Dib", 7)?;

        state.serialize_field("Dif byte", &byte_to_hex_string(&self.dif_byte))?;

        if self.dife_bytes.is_empty() {
            state.serialize_field("Dife bytes", &Option::<String>::None)?;
        }
        else {
            state.serialize_field("Dife bytes", &array_bytes_to_hex_string(&self.dife_bytes))?;
        }

        state.serialize_field("Storage number", &self.storage_number)?;
        state.serialize_field("Tariff", &self.tariff)?;
        state.serialize_field("Subunit", &self.subunit)?;

        match &self.function_field {
            Some(x) => {
                state.serialize_field("Function field", &x.to_string())?;
            },
            None => {
                state.serialize_field("Function field", &Option::<String>::None)?;
            }
        }

        state.serialize_field("Data type", &self.data_type.to_string())?;

        state.end()
    }
}

impl Dib {
    pub fn new(data: &mut VecDeque<u8>) -> Result<Dib, ParserError> {
        if data.len() < 1 {
            return Err(ParserError::DibCalculatorError);
        }

        let dif_byte: u8 = data.pop_front().unwrap();
        let data_type: DibDataType = DibDataType::new(dif_byte);
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

        dib.function_field = Some(DibFunctionField::new(dib.dif_byte));
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
            let storage_number_loop: u32 = (dife_byte & 0x0F) as u32;

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
    fn new(byte: u8) -> DibFunctionField {
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
    pub fn new(byte: u8) -> DibDataType {
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
}

#[cfg(test)]
#[path = "tests/tests_dib.rs"]
mod tests_dib_calculator;