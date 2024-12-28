use std::collections::VecDeque;
use crate::tools::checksum::*;
use crate::tools::tools::*;
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
            return Err(ParserError::TelegramFormatNotSupported);
        }

        if data.len() == 5 && data[0] == 0x10 && data[4] == 0x16 {
            Self::validate_cs(&data[1..data.len() -1].to_vec())?;
            return Err(ParserError::TelegramFormatNotSupported);
        }

        if data.len() > 5 && data.len() < 262 && data[0] == 0x68 && data[3] == 0x68 && data[1] == data[2] && data[data.len() - 1] == 0x16 {
            Self::validate_length_mbus(&data)?;
            Self::validate_cs(&data[4..data.len() -1].to_vec())?;

            if data[1] == 0x03 {
                return Err(ParserError::TelegramFormatNotSupported);
            }
            if data[1] != 0x03 {
                return Ok(TelegramFormat::LongFrameMBus);
            }

            return Err(ParserError::TelegramFormatCalculatorError);
        }

        if data.len() > 5 && data[1] == 0x44 {
            if data.len() < 291 && Self::validate_length_wmbus_format_a(&data).is_ok() && Self::validate_crc_wmbus_format_a(&data).is_ok() {
                return Ok(TelegramFormat::LongFrameWMBusFormatA);
            }
            if data.len() < 257 && Self::validate_length_wmbus_format_b(&data).is_ok() && Self::validate_crc_wmbus_format_b(&data).is_ok() {
                return Ok(TelegramFormat::LongFrameWMBusFormatB);
            }
            if Self::validate_length_wmbus_format_a(&data).is_err() || Self::validate_length_wmbus_format_b(&data).is_err() {
                return Err(ParserError::WmbusInvalidDatagramLength);
            }
            if Self::validate_length_wmbus_format_a(&data).is_ok() && Self::validate_crc_wmbus_format_a(&data).is_err() {
                Self::validate_crc_wmbus_format_a(&data)?;
            }
            if Self::validate_length_wmbus_format_b(&data).is_ok() && Self::validate_crc_wmbus_format_b(&data).is_err() {
                Self::validate_crc_wmbus_format_b(&data)?;
            }
        }

        return Err(ParserError::TelegramFormatCalculatorError);
    }

    fn validate_length_mbus(data: &Vec<u8>) -> Result<(), ParserError> {
        let length: i16 = data.len() as i16;

        if length -6 != data[1] as i16 {
            return Err(ParserError::MbusInvalidDatagramLength);
        }

        return Ok(());
    }
    
    fn validate_length_wmbus_format_a(data: &Vec<u8>) -> Result<(), ParserError> {
        let mut intermediate_length: i16 = data[0] as i16;
        intermediate_length -= 9;
        let remaining_blocks: u8 = (intermediate_length as f32 / 16 as f32).ceil() as u8;
        let number_of_crc_bytes: u8 = (remaining_blocks * 2) + 2;
        let length: i16 = data.len() as i16 - number_of_crc_bytes as i16 - 1; 

        if length != data[0] as i16 {
            return Err(ParserError::WmbusInvalidDatagramLength);
        }

        return Ok(());
    }
    
    fn validate_length_wmbus_format_b(data: &Vec<u8>) -> Result<(), ParserError> {
        let length: i16 = data.len() as i16;

        if length -1 != data[0] as i16 {
            return Err(ParserError::WmbusInvalidDatagramLength);
        }

        return Ok(());
    }
    
    fn validate_cs(data: &Vec<u8>) -> Result<(), ParserError> {
        let given_cs: u8 = data[data.len() -1];
        let calculated_cs: u8 = calculate_mbus_cs(&data[..data.len() -1])?;
    
        if given_cs != calculated_cs {
            return Err(ParserError::MbusInvalidChecksum(format!("{} vs {}", byte_to_hex_string(&given_cs), byte_to_hex_string(&calculated_cs))));
        }
    
        return Ok(());
    }
    
    fn validate_crc_wmbus_format_a(data: &Vec<u8>) -> Result<(), ParserError> {
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(data.iter());
    
        if buffer.len() < 12 {
            return Err(ParserError::WmbusInvalidCrc(format!("Insufficient data for calculate crc")));
        }
    
        let data_block: Vec<u8> = buffer.drain(..10).collect();
        let given_crc: [u8; 2]  = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
        let calculated_crc: [u8; 2] = calculate_wmbus_crc(&data_block)?;

        if given_crc != calculated_crc {
            return Err(ParserError::WmbusInvalidCrc(format!("{} vs {}", array_bytes_to_hex_string(&given_crc).unwrap_or_default(), array_bytes_to_hex_string(&calculated_crc).unwrap_or_default())));
        }
        
        while buffer.len() >= 18 {
            let data_block: Vec<u8> = buffer.drain(..16).collect();
            let given_crc: [u8; 2]  = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
            let calculated_crc: [u8; 2] = calculate_wmbus_crc(&data_block)?;

            if given_crc != calculated_crc {
                return Err(ParserError::WmbusInvalidCrc(format!("{} vs {}", array_bytes_to_hex_string(&given_crc).unwrap_or_default(), array_bytes_to_hex_string(&calculated_crc).unwrap_or_default())));
            }
        }
        
        let remaining_number_of_bytes: usize = buffer.len();
    
        if remaining_number_of_bytes == 0 {
            return Ok(());
        }
    
        if remaining_number_of_bytes < 3 {
            return Err(ParserError::WmbusInvalidCrc(format!("Insufficient data for calculate crc")));
        }
    
        let data_block: Vec<u8> = buffer.drain(..remaining_number_of_bytes -2).collect();
        let given_crc: [u8; 2]  = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
        let calculated_crc: [u8; 2] = calculate_wmbus_crc(&data_block)?;

        if given_crc != calculated_crc {
            return Err(ParserError::WmbusInvalidCrc(format!("{} vs {}", array_bytes_to_hex_string(&given_crc).unwrap_or_default(), array_bytes_to_hex_string(&calculated_crc).unwrap_or_default())));
        }
    
        return Ok(());
    }
    
    fn validate_crc_wmbus_format_b(data: &Vec<u8>) -> Result<(), ParserError>  {
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(data.iter());
    
        if buffer.len() < 10 {
            return Err(ParserError::WmbusInvalidCrc(format!("Insufficient data for calculate crc")));
        }
    
        buffer.drain(..10);
        
        if buffer.len() >= 118 {
            let data_block: Vec<u8> = buffer.drain(..116).collect();
            let given_crc: [u8; 2]  = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
            let calculated_crc: [u8; 2] = calculate_wmbus_crc(&data_block)?;

            if given_crc != calculated_crc {
                return Err(ParserError::WmbusInvalidCrc(format!("{} vs {}", array_bytes_to_hex_string(&given_crc).unwrap_or_default(), array_bytes_to_hex_string(&calculated_crc).unwrap_or_default())));
            }
        }
        
        let remaining_number_of_bytes: usize = buffer.len();
    
        if remaining_number_of_bytes == 0 {
            return Ok(());
        }
    
        if remaining_number_of_bytes < 3 {
            return Err(ParserError::WmbusInvalidCrc(format!("Insufficient data for calculate crc")));
        }
    
        let data_block: Vec<u8> = buffer.drain(..remaining_number_of_bytes -2).collect();
        let given_crc: [u8; 2]  = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
        let calculated_crc: [u8; 2] = calculate_wmbus_crc(&data_block)?;

        if given_crc != calculated_crc {
            return Err(ParserError::WmbusInvalidCrc(format!("{} vs {}", array_bytes_to_hex_string(&given_crc).unwrap_or_default(), array_bytes_to_hex_string(&calculated_crc).unwrap_or_default())));
        }
    
        return Ok(());
    }
}

#[cfg(test)]
#[path = "tests/tests_telegram_format.rs"]
mod tests_telegram_format_calculator;