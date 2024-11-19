use std::collections::VecDeque;
use crate::tools::checksum::*;
use crate::matrices::errors::*;

#[derive(Debug, PartialEq)]
pub enum TelegramFormat {
    LongFrameMBus,
    LongFrameWMBusFormatA,
    LongFrameWMBusFormatB,
}

impl TelegramFormat {
    pub fn calculate_telegram_format(data: &Vec<u8>) -> Result<Self, ParserError> {
        if data.is_empty() {
            return Err(ParserError::TelegramFormatCalculatorError);
        }

        if data.len() == 1 && data[0] == 0xE5 {
            return Err(ParserError::TelegramFormatCalculatorError);
        }

        if data.len() == 5 && data[0] == 0x10 && data[4] == 0x16 {
            if Self::validate_cs(&data[1..data.len() -1].to_vec()) {
                return Err(ParserError::TelegramFormatCalculatorError);
            }
        }

        if data.len() > 5 && data.len() < 10 && data[0] == 0x68 && data[3] == 0x68 && data[data.len() - 1] == 0x16 {
            if data[1] == 0x03 && data[2] == 0x03 {
                if Self::validate_length_mbus(&data) {
                    if Self::validate_cs(&data[4..data.len() -1].to_vec()) {
                        return Err(ParserError::TelegramFormatCalculatorError);
                    }
                }
            }
        }

        if data.len() > 5 && data.len() < 262 && data[0] == 0x68 && data[3] == 0x68 && data[data.len() - 1] == 0x16 {
            if data[1] != 0x03 && data[2] != 0x03 && data[1] == data[2] {
                if Self::validate_length_mbus(&data) {
                    if Self::validate_cs(&data[4..data.len() -1].to_vec()) {
                        return Ok(TelegramFormat::LongFrameMBus);
                    }
                }
            }
        }

        if data.len() > 5 && data.len() < 291 && data[1] == 0x44{
            if Self::validate_length_wmbus_format_a(&data) {
                if Self::validate_crc_wmbus_format_a(&data) {
                    return Ok(TelegramFormat::LongFrameWMBusFormatA);
                }
            }
        }

        if data.len() > 5 && data.len() < 257 && data[1] == 0x44{
            if Self::validate_length_wmbus_format_b(&data) {
                if Self::validate_crc_wmbus_format_b(&data) {
                    return Ok(TelegramFormat::LongFrameWMBusFormatB);
                }
            }
        }

        return Err(ParserError::TelegramFormatCalculatorError);
    }

    fn validate_length_mbus(data: &Vec<u8>) -> bool {
        let length: i16 = data.len() as i16;
        return length -6 == data[1] as i16;
    }
    
    fn validate_length_wmbus_format_a(data: &Vec<u8>) -> bool {
        let mut intermediate_length: i16 = data[0] as i16;
        intermediate_length -= 9;
        let remaining_blocks: u8 = (intermediate_length as f32 / 16 as f32).ceil() as u8;
        let number_of_crc_bytes: u8 = (remaining_blocks * 2) + 2;
        let length: i16 = data.len() as i16 - number_of_crc_bytes as i16 - 1; 
        return length == data[0] as i16;
    }
    
    fn validate_length_wmbus_format_b(data: &Vec<u8>) -> bool {
        let length: i16 = data.len() as i16;
        return length -1 == data[0] as i16;
    }
    
    fn validate_cs(data: &Vec<u8>) -> bool {
        let given_cs: u8 = data[data.len() -1];
        let calculated_cs: Option<u8> = calculate_mbus_cs(&data[..data.len() -1].to_vec());
    
        if calculated_cs.is_some() {
            return given_cs == calculated_cs.unwrap();
        }
    
        return false;
    }
    
    fn validate_crc_wmbus_format_a(data: &Vec<u8>) -> bool {
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(data.iter());
    
        if buffer.len() < 12 {
            return false;
        }
    
        let data_block: Vec<u8> = buffer.drain(..10).collect();
        let given_crc: Vec<u8> = buffer.drain(..2).collect();
    
        let calculated_crc: Option<Vec<u8>> = calculate_wmbus_crc(&data_block);
        if calculated_crc.is_some() {
            if given_crc != calculated_crc.unwrap() {
                return false;
            }
        }
        else {
            return false;
        }
        
        while buffer.len() >= 18 {
            let data_block: Vec<u8> = buffer.drain(..16).collect();
            let given_crc: Vec<u8> = buffer.drain(..2).collect();
    
            let calculated_crc: Option<Vec<u8>> = calculate_wmbus_crc(&data_block);
            if calculated_crc.is_some() {
                if given_crc != calculated_crc.unwrap() {
                    return false;
                }
            }
            else {
                return false;
            }
        }
        
        let remaining_number_of_bytes: usize = buffer.len();
    
        if remaining_number_of_bytes == 0 {
            return true;
        }
    
        if remaining_number_of_bytes < 3 {
            return false;
        }
    
        let data_block: Vec<u8> = buffer.drain(..remaining_number_of_bytes -2).collect();
        let given_crc: Vec<u8> = buffer.drain(..2).collect();
    
        let calculated_crc: Option<Vec<u8>> = calculate_wmbus_crc(&data_block);
        if calculated_crc.is_some() {
            if given_crc != calculated_crc.unwrap() {
                return false;
            }
        }
        else {
            return false;
        }
    
        return true;
    }
    
    fn validate_crc_wmbus_format_b(data: &Vec<u8>) -> bool {
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(data.iter());
    
        if buffer.len() < 10 {
            return false;
        }
    
        buffer.drain(..10);
        
        if buffer.len() >= 118 {
            let data_block: Vec<u8> = buffer.drain(..116).collect();
            let given_crc: Vec<u8> = buffer.drain(..2).collect();
    
            let calculated_crc: Option<Vec<u8>> = calculate_wmbus_crc(&data_block);
            if calculated_crc.is_some() {
                if given_crc != calculated_crc.unwrap() {
                    return false;
                }
            }
            else {
                return false;
            }
        }
        
        let remaining_number_of_bytes: usize = buffer.len();
    
        if remaining_number_of_bytes == 0 {
            return true;
        }
    
        if remaining_number_of_bytes < 3 {
            return false;
        }
    
        let data_block: Vec<u8> = buffer.drain(..remaining_number_of_bytes -2).collect();
        let given_crc: Vec<u8> = buffer.drain(..2).collect();
    
        let calculated_crc: Option<Vec<u8>> = calculate_wmbus_crc(&data_block);
        if calculated_crc.is_some() {
            if given_crc != calculated_crc.unwrap() {
                return false;
            }
        }
        else {
            return false;
        }
    
        return true;
    }
}

#[cfg(test)]
#[path = "tests/tests_telegram_format.rs"]
mod tests_telegram_format_calculator;