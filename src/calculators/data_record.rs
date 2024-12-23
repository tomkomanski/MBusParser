use std::collections::VecDeque;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::calculators::dib::*;
use crate::calculators::lvar::*;
use crate::calculators::vib::*;
use crate::matrices::errors::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
pub struct DataRecord {
    pub record_number: u8,
    pub dib: Dib,
    pub vib: Option<Vib>,
    pub data: Option<Vec<u8>>,
    pub numeric_value: Option<f64>,
    pub text_value: Option<String>,
    pub comment: Option<String>,
}

impl Serialize for DataRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Data records", 7)?;

        state.serialize_field("Record number", &self.record_number)?;
        state.serialize_field("Dib", &self.dib)?;
        state.serialize_field("Vib", &self.vib)?;

        match &self.data {
            Some(x) => {
                state.serialize_field("Data", &array_bytes_to_hex_string(x))?;
            },
            None => {
                state.serialize_field("Data", &Option::<Vec<u8>>::None)?;
            }
        }

        state.serialize_field("Numeric value", &self.numeric_value)?;
        state.serialize_field("Text value", &self.text_value)?;
        state.serialize_field("Comment", &self.comment)?;

        state.end()
    }
}

impl DataRecord {
    pub fn calculate_data_record(data: &mut VecDeque<u8>, manufacturer: &Option<[u8; 2]>) -> Result<Option<DataRecord>, ParserError>{
        let dib: Result<Dib, ParserError> = Dib::new(data);
        if dib.is_err() {
            return Err(dib.unwrap_err());
        }

        let mut data_record: DataRecord = DataRecord {
            record_number: 0,
            dib: dib.unwrap(),
            vib: None,
            data: None,
            numeric_value: None,
            text_value: None,
            comment: None,
        };

        if data_record.dib.data_type == DibDataType::SpecialFunctionIdleFilter {
            return Ok(None);
        }

        if data_record.dib.data_type == DibDataType::SpecialFunctionManufacturerSpecific || 
            data_record.dib.data_type == DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram ||
            data_record.dib.data_type == DibDataType::SpecialFunctionReserved0x3F || 
            data_record.dib.data_type == DibDataType::SpecialFunctionReserved0x4F || 
            data_record.dib.data_type == DibDataType::SpecialFunctionReserved0x5F || 
            data_record.dib.data_type == DibDataType::SpecialFunctionReserved0x6F ||
            data_record.dib.data_type == DibDataType::SpecialFunctionGlobalReadout {

            let data: Vec<u8> = data.drain(..).collect();
            let data_hex: String = array_bytes_to_hex_string(&data);

            data_record.data = Some(data);
            data_record.text_value = Some(data_hex);

            return Ok(Some(data_record));
        }

        let vib: Result<Vib, ParserError> = Vib::new(data, manufacturer);
        if vib.is_err() {
            return Err(vib.unwrap_err());
        }
        data_record.vib = Some(vib.unwrap());

        if data.len() < data_record.dib.data_length as usize {
            return Err(ParserError::DataRecordCalculatorError);
        }

        if data_record.dib.data_type == DibDataType::NoData || data_record.dib.data_type == DibDataType::SelectionForReadout {
            return Ok(Some(data_record));
        }

        let data_record_data: Vec<u8> = data.drain(..data_record.dib.data_length as usize).collect();
        data_record.data = Some(data_record_data.clone());

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeG) && data_record.dib.data_type == DibDataType::Data16BitInteger {
            if data_record_data[0] == 0xFF && data_record_data[1] == 0xFF {
                data_record.comment = data_record.comment.concatenate_str("Invalid date value");
            }
            else {
                let day: u8 = data_record_data[0] & 0x1F;
                let month: u8 = data_record_data[1] & 0x0F;
                let mut year: u16 = (((data_record_data[0] & 0xE0) >> 5) | ((data_record_data[1] & 0xF0) >> 1)) as u16;
                if year <= 80 {
                    year += 2000; 
                }
                else {
                    year += 1900;
                }
                
                data_record.text_value = Some(format!("{}-{:02}-{:02}", year, month, day));
            }

            return Ok(Some(data_record));
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeFJIM) && data_record.dib.data_type == DibDataType::Data24BitInteger {
            //PN-EN 13757-3 2018
            if data_record_data[0] == 0xFF && data_record_data[1] == 0xFF && data_record_data[2] == 0xFF{
                data_record.comment = data_record.comment.concatenate_str("Invalid date value");
            }
            //PN-EN 13757-3 2013
            else if data_record_data[0] == 0x00 && data_record_data[1] == 0x00 && data_record_data[2] == 0x00{
                data_record.comment = data_record.comment.concatenate_str("Invalid date value");
            }
            else {
                let second: u8 = data_record_data[0] & 0x3F;
                let minute: u8 = data_record_data[1] & 0x3F;
                let hour: u8 = data_record_data[2] & 0x1F;
            
                data_record.text_value = Some(format!("{}:{:02}:{:02}", hour, minute, second));
            }

            return Ok(Some(data_record));
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeFJIM) && data_record.dib.data_type == DibDataType::Data32BitInteger {                  
            let iv: u8 = (data_record_data[0] & 0x80) >> 7;
            if iv == 1 {
                data_record.comment = data_record.comment.concatenate_str("Invalid date value");
            }
            else {
                let summer_time_tag: u8 = (data_record_data[1] & 0x80) >> 7;
                if summer_time_tag == 1 {
                    data_record.comment = data_record.comment.concatenate_str("Summer time");
                }
                let minute: u8 = data_record_data[0] & 0x3F;
                let hour: u8 = data_record_data[1] & 0x1F;
                let day: u8 = data_record_data[2] & 0x1F;
                let month: u8 = data_record_data[3] & 0x0F;
                let mut year: u16 = (((data_record_data[2] & 0xE0) >> 5) | ((data_record_data[3] & 0xF0) >> 1)) as u16;
                if year <= 80 {
                   year += 2000; 
                }
                else {
                    let hundred_year: u16 = ((data_record_data[1] & 0x3F) >> 5) as u16;
                    year += 1900 + (100 * hundred_year);
                }

                data_record.text_value = Some(format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, 0));
            }

            return Ok(Some(data_record));
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeFJIM) && data_record.dib.data_type == DibDataType::Data48BitInteger {
            let iv: u8 = (data_record_data[0] & 0x80) >> 7;
            if iv == 1 {
                data_record.comment = data_record.comment.concatenate_str("Invalid date value");
            }
            else {
                let summer_time_tag: u8 = (data_record_data[0] & 0x40) >> 6;
                if summer_time_tag == 1 {
                    data_record.comment = data_record.comment.concatenate_str("Summer time");
                }
                let second: u8 = data_record_data[0] & 0x3F;
                let minute: u8 = data_record_data[1] & 0x3F;
                let hour: u8 = data_record_data[2] & 0x1F;
                let day: u8 = data_record_data[3] & 0x1F;
                let month: u8 = data_record_data[4] & 0x0F;
                let year: u16 = 2000 + (((data_record_data[3] & 0xE0) >> 5) | ((data_record_data[4] & 0xF0) >> 1)) as u16;

                data_record.text_value = Some(format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second));
            }

            return Ok(Some(data_record));
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeFJIM) && data_record.dib.data_type == DibDataType::DataVariableLength {
            let lvar: Result<Lvar, ParserError> = Lvar::new(data_record_data[0], data);
            if lvar.is_err() {
                return Err(lvar.unwrap_err());
            }
            let lvar: Lvar = lvar.unwrap();

            data_record.data = Some(lvar.data);
            data_record.numeric_value = lvar.numeric_value;
            data_record.text_value = lvar.text_value;

            return Ok(Some(data_record));
        }

        if (data_record.vib.as_ref().unwrap().data_type.as_ref().is_none() || 
                data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x != &VibDataType::DataTypeFJIM)) && 
                data_record.dib.data_type == DibDataType::DataVariableLength {

            let lvar: Result<Lvar, ParserError> = Lvar::new(data_record_data[0], data);
            if lvar.is_err() {
                return Err(lvar.unwrap_err());
            }
            let lvar: Lvar = lvar.unwrap();

            data_record.data = Some(lvar.data);
            data_record.numeric_value = lvar.numeric_value;
            data_record.text_value = lvar.text_value;

            return Ok(Some(data_record));
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::StandardConformData) {
            // -----
            // To Do
            //------
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::CompactProfileWithRegister) {
            // -----
            // To Do
            //------
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::CompactProfile) {
            // Do nothing, processed with a separate calculator call (compact profiles calculator)
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::WirelessMbusDataContainer) {
            // Do nothing, processed with a separate calculator call (wireless mbus data container post processing)
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::ManufacturerSpecificDataContainer) {
            // Do nothing, processed with a separate calculator call (manufacturer data container post processing)
        }

        if data_record.vib.as_ref().unwrap().data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::Numeric) {
            let dataval: Result<Option<f64>, ParserError> = Self::calculate_data(&data_record.dib.data_type, &data_record_data);
            if dataval.is_err() {
                return Err(ParserError::DataRecordCalculatorError);
            }
            let dataval: Option<f64> = dataval.unwrap();
            if dataval.is_none() {
                return Err(ParserError::DataRecordCalculatorError);
            }
            let dataval: f64 = dataval.unwrap();

            let magnitude: Option<i8> = data_record.vib.as_ref().unwrap().magnitude;
            if magnitude.is_none() {
                return Err(ParserError::DataRecordCalculatorError);
            }
            let magnitude: i8 = magnitude.unwrap();

            let base: f64 = 10.0;
            let mut data_val: f64 = dataval * base.powf(magnitude as f64);

            // round if data is not i64 and multiplier <= 10 and multiplier >= 0.001
            if data_record.dib.data_type != DibDataType::Data64BitInteger && magnitude <= 1 && magnitude >= -3 {
                data_val = (data_val * 1000.0).round() / 1000.0;
            }

            data_record.numeric_value = Some(data_val);
            if data_val.is_nan() {
                data_record.comment = data_record.comment.concatenate_str("Numeric value is NaN");
            }
            return Ok(Some(data_record));
        }
        
        data_record.text_value = Some(array_bytes_to_hex_string(&data_record_data));
        return Ok(Some(data_record));
    }

    pub fn calculate_data(data_type: &DibDataType, bytes: &[u8]) -> Result<Option<f64>, ParserError> {
        match data_type {
            DibDataType::NoData => {
                if bytes.len() != 0 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                return Ok(None);
            }
            DibDataType::Data8BitInteger => {
                if bytes.len() != 1 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = i8::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            }
            DibDataType::Data16BitInteger => {
                if bytes.len() != 2 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 2], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = i16::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data24BitInteger => {
                if bytes.len() != 3 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 3], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_24_to_int_32(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data32BitInteger => {
                if bytes.len() != 4 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = i32::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data32BitReal => {
                if bytes.len() != 4 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = f32::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data48BitInteger => {
                if bytes.len() != 6 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 6], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_48_to_int_64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data64BitInteger => {
                if bytes.len() != 8 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 8], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = i64::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::SelectionForReadout => {
                if bytes.len() != 0 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                return Ok(None);
            },
            DibDataType::Data2DigitBCD => {
                if bytes.len() != 1 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data4DigitBCD => {
                if bytes.len() != 2 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 2], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data6DigitBCD => {
                if bytes.len() != 3 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 3], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data8DigitBCD => {
                if bytes.len() != 4 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 4], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = array_bcd_to_u64(&bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::DataVariableLength => {
                if bytes.len() != 1 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 1], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let value: f64 = u8::from_le_bytes(bytes.unwrap()) as f64;

                return Ok(Some(value));
            },
            DibDataType::Data12DigitBCD => {
                if bytes.len() != 6 {
                    return Err(ParserError::DataRecordCalculatorError);
                }

                let bytes: Result<[u8; 6], _> = bytes.try_into();
                if bytes.is_err() {
                    return Err(ParserError::DataRecordCalculatorError);
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

    pub fn update_data_record_text_value(&mut self, str: &str) {
        self.text_value = Some(str.to_string());
    }
}

#[cfg(test)]
#[path = "tests/tests_data_record.rs"]
mod tests_data_record_calculator;