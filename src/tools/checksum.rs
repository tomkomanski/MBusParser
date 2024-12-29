use crate::matrices::errors::*;

pub fn calculate_mbus_cs(data: &[u8]) -> Result<u8, ParserError> {
    if data.len() > 0 {
        let mut cs: u8 = 0;

        data.iter().for_each(|n: &u8| {
            cs = cs.wrapping_add(*n);
        });
        return Ok(cs);
    }
    return Err(ParserError::MbusChecksumCalculationError);
}

pub fn calculate_wmbus_crc(data: &[u8]) -> Result<[u8; 2], ParserError> {
    if data.len() > 2 {
        let poly: u16 = 0x3D65;
        let xor_value: u16 = 0xFFFF;
        let mut crc_val: u16 = 0x0000;

        data.iter().for_each(|n: &u8| {
            let mut i: u8 = 0x80;
            while i != 0 {
                if (crc_val & 0x8000) != 0
                {
                    crc_val = (crc_val << 1) ^ poly;
                }
                else
                {
                    crc_val = crc_val << 1;
                }
                if (*n & i) != 0
                {
                    crc_val ^= poly;
                }
                i >>= 1;
            }  
        });

        let tmp_crc: u16 = crc_val & 0xFFFF ^ xor_value;
        let crc: [u8; 2] = tmp_crc.to_be_bytes();
        return Ok(crc);
    }

    return Err(ParserError::WmbusCrcCalculationError);
}

#[cfg(test)]
#[path = "tests/tests_checksum.rs"]
mod tests_checksum;