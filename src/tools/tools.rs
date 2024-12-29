use std::str;

pub fn bcd_to_u64(arr: &[u8]) -> u64 {
    let mut output: u64 = 0;

    arr.iter().rev().for_each(|n: &u8| {
        output = (output * 10) + ((n >> 4) & 0xF) as u64;
        output = (output * 10) + (n & 0xF) as u64;
    });

    return output;
}

pub fn array_24_to_int_32(arr: [u8; 3]) -> i32 {
    if (arr[2] & 0x80) >> 7 == 1 {
        return (((0xFF as u32) << 24) | ((arr[2] as u32) << 16) | ((arr[1] as u32) << 8) | (arr[0] as u32)) as i32;
    }
    else {
        return (((arr[2] as u32) << 16) | ((arr[1] as u32) << 8) | (arr[0] as u32)) as i32;
    }
}

pub fn array_48_to_int_64(arr: [u8; 6]) -> i64 {
    if (arr[5] & 0x80) >> 7 == 1 {
        return (((0xFF as u64) << 56) | ((0xFF as u64) << 48) | ((arr[5] as u64) << 40) | ((arr[4] as u64)<< 32) | ((arr[3] as u64) << 24) | ((arr[2] as u64) << 16) | ((arr[1] as u64) << 8) | (arr[0] as u64)) as i64;
    }
    else {
        return (((arr[5] as u64) << 40) | ((arr[4] as u64) << 32) | ((arr[3] as u64) << 24) | ((arr[2] as u64) << 16) | ((arr[1] as u64) << 8) | (arr[0] as u64)) as i64;
    }
}

pub fn hex_string_to_byte_vector(input: &str) -> Option<Vec<u8>> {
    if input.is_empty() {
        return None;
    }

    if !input.is_ascii() {
        return None;
    }

    let input: &str = input.trim();
    let cleaned_data: String = input.replace("0x", "").replace("0X", "").replace([' ', ',', '-'], "");

    if cleaned_data.chars().count() % 2 != 0 {
        return None;
    }

    let data_chunks: std::slice::Chunks<u8> = cleaned_data.as_bytes().chunks(2);

    let mut output: Vec<u8> = Vec::new();
    for slice in data_chunks {
        let byte_str: &str = match str::from_utf8(slice) {
            Ok(str) => str,
            Err(_) => {
                return None;
            }
        };

        let byte: u8 = match u8::from_str_radix(byte_str, 16) {
            Ok(bt) => bt,
            Err(_) => {
                return None;
            }
        };

        output.push(byte);
    }

    return Some(output);
}

pub fn byte_to_hex_string(input: &u8) -> String {
    let output: String = String::from("0x") + format!("{:02x}", input).to_uppercase().as_str();

    return output;
}

pub fn array_bytes_to_hex_string(input: &[u8]) -> Option<String> {
    
    if !input.is_empty() {
        let mut output: String = String::new();

        for byte_opt in input.iter() {
            output = output + "0x" + format!("{:02x}", byte_opt).to_uppercase().as_str() + " ";
        }

        let output: String = output.trim().to_string();
        return Some(output);
    }

    return None;
}

pub trait OptionStringExt {
    fn concatenate_str(&self, str: &str) -> Option<String>;
}

impl OptionStringExt for Option<String> {
    fn concatenate_str(&self, str: &str) -> Option<String> {
        if self.is_none() {
            return Some(str.to_string());
        }
        else {
            return Some((self.as_ref().unwrap().to_string() + " " + str).trim().to_string());
        }
    }
}

#[cfg(test)]
#[path = "tests/tests_tools.rs"]
mod tests_tools;