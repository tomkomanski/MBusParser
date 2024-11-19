use std::collections::VecDeque;
use crate::matrices::errors::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
enum LvarDataType {
    TextString,
    PositiveBCDnumber,
    NegativeBCDnumber,
    BinaryNumber,
}

#[derive(Debug, PartialEq)]
struct LvarDataTypeAndLength {
    data_type: LvarDataType,
    data_length: u8,
}

#[derive(Debug, PartialEq)]
pub struct Lvar {
    pub data: Vec<u8>,
    pub numeric_value: Option<f64>,
    pub text_value: Option<String>,
}

impl Default for Lvar {
    fn default() -> Self {
        Lvar {
            data: Vec::new(),
            numeric_value: None,
            text_value: None,
        }
    }
}

impl LvarDataTypeAndLength {
    fn new (data: u8) -> Result<Self, ParserError> {
        match data {
            0x00..=0xBF => {
                let length: u8 = data;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::TextString,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xC0..=0xC9 => {
                let length: u8 = data - 0xC0;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::PositiveBCDnumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xD0..=0xD9 => {
                let length: u8 = data - 0xD0;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::NegativeBCDnumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xE0..=0xEF => {
                let length: u8 = data - 0xE0;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::BinaryNumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xF0..=0xF4 => {
                let length: u8 = (data - 0xEC) * 4;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::BinaryNumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xF5 => {
                let length: u8 = 6;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::BinaryNumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            0xF6 => {
                let length: u8 = 8;
                let lvar_data_type_and_length: LvarDataTypeAndLength = LvarDataTypeAndLength {
                    data_type: LvarDataType::BinaryNumber,
                    data_length: length,
                };
                return Ok(lvar_data_type_and_length);
            },
            _ => {
                return Err(ParserError::LvarError);
            }
        };
    }
}

impl Lvar {
    pub fn new (lvar_first_byte: u8, data: &mut VecDeque<u8>) -> Result<Self, ParserError> {
        let lvar_data_type_and_length: Result<LvarDataTypeAndLength, ParserError> = LvarDataTypeAndLength::new(lvar_first_byte);
        if lvar_data_type_and_length.is_err() {
            return Err(lvar_data_type_and_length.unwrap_err());
        }
        let lvar_data_type_and_length: LvarDataTypeAndLength = lvar_data_type_and_length.unwrap();

        if data.len() < lvar_data_type_and_length.data_length as usize{
            return Err(ParserError::LvarError);
        }

        let mut lvar: Lvar = Lvar {..Lvar::default() };

        let value_bytes: Vec<u8> = data.drain(..lvar_data_type_and_length.data_length as usize).collect();
        let mut data_bytes: Vec<u8> = Vec::new();
        data_bytes.push(lvar_first_byte);
        data_bytes.extend(value_bytes.clone());

        lvar.data = data_bytes;

        if lvar_data_type_and_length.data_type == LvarDataType::TextString {
            let value_bytes_reversed: Vec<u8> = value_bytes.iter().copied().rev().collect();
            let value_text: Result<String, std::string::FromUtf8Error> = String::from_utf8(value_bytes_reversed);

            if value_text.is_ok() {
                lvar.text_value = Some(value_text.unwrap());
            }
            else {
                lvar.text_value = Some(array_bytes_to_hex_string(&value_bytes));
            }
        }
        else if lvar_data_type_and_length.data_type == LvarDataType::PositiveBCDnumber &&
                lvar_data_type_and_length.data_length >= 1 && lvar_data_type_and_length.data_length <= 9 {

            lvar.numeric_value = Some(array_bcd_to_u64(&value_bytes) as f64);
        }
        else if lvar_data_type_and_length.data_type == LvarDataType::NegativeBCDnumber && 
                lvar_data_type_and_length.data_length >= 1 && lvar_data_type_and_length.data_length <= 9 {

            lvar.numeric_value = Some((array_bcd_to_u64(&value_bytes) as f64) * -1.0);
        }
        else if lvar_data_type_and_length.data_type == LvarDataType::BinaryNumber {
            let value_bytes_reversed: Vec<u8> = value_bytes.iter().copied().rev().collect();

            let mut binary_output: String = String::new();
            for n in 0..value_bytes_reversed.len() {
                let binary: String = format!("{:b}", value_bytes_reversed[n]);
                binary_output += binary.as_str();
            }

            lvar.text_value = Some(binary_output);
        }
                
        return Ok(lvar);
    }
}

#[cfg(test)]
#[path = "tests/tests_lvar.rs"]
mod tests_lvar_calculator;