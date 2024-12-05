use crate::matrices::errors::*;

pub fn calculate_mbus_cs(data: &[u8]) -> Result<u8, ParserError> {
    if data.len() > 0 {
        let mut cs: u8 = 0;

        for n in 0..data.len() {
            cs = cs.wrapping_add(data[n]);
        }
        return Ok(cs);
    }
    return Err(ParserError::MbusChecksumCalculationError);
}

pub fn calculate_wmbus_crc(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() > 2 {
        let poly: u16 = 0x3D65;
        let xor_value: u16 = 0xFFFF;
        let mut crc_val: u16 = 0x0000;

        for n in 0..data.len() {
            let b: u8 = data[n];
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
                if (b & i) != 0
                {
                    crc_val ^= poly;
                }
                i >>= 1;
            }         
        }

        let tmp_crc: u16 = crc_val & 0xFFFF ^ xor_value;
        let crc: Vec<u8> = tmp_crc.to_be_bytes().to_vec();
        return Some(crc);
    }

    return None;
}

#[cfg(test)]
#[path = "tests/tests_checksum.rs"]
mod tests_checksum;