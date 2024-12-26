use super::super::tools::*;

#[test]
fn test_001_array_bcd_to_u64() {
    let bytes: [u8; 1] = [0x55];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 55);
}
#[test]
fn test_002_array_bcd_to_u64() {
    let bytes: [u8; 1] = [0x03];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3);
}
#[test]
fn test_003_array_bcd_to_u64() {
    let bytes: [u8; 1] = [0xB0];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 110);
}
#[test]
fn test_004_array_bcd_to_u64() {
    let bytes: [u8; 2] = [0x03, 0x00];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3);
}
#[test]
fn test_005_array_bcd_to_u64() {
    let bytes: [u8; 2] = [0x03, 0x30];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3003);
}
#[test]
fn test_006_array_bcd_to_u64() {
    let bytes: [u8; 2] = [0xA3, 0x30];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3103);
}
#[test]
fn test_007_array_bcd_to_u64() {
    let bytes: [u8; 3] = [0x03, 0x30, 0x03];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 33003);
}
#[test]
fn test_008_array_bcd_to_u64() {
    let bytes: [u8; 3] = [0x63, 0x32, 0x43];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 433263);
}
#[test]
fn test_009_array_bcd_to_u64() {
    let bytes: [u8; 3] = [0xF3, 0x3D, 0x43];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 434453);
}
#[test]
fn test_010_array_bcd_to_u64() {
    let bytes: [u8; 4] = [0x50, 0x56, 0x00, 0x00];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 5650);
}
#[test]
fn test_011_array_bcd_to_u64() {
    let bytes: [u8; 4] = [0x12, 0x11, 0x07, 0x03];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3071112);
}
#[test]
fn test_012_array_bcd_to_u64() {
    let bytes: [u8; 4] = [0x09, 0x05, 0x80, 0x98];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 98800509);
}
#[test]
fn test_013_array_bcd_to_u64() {
    let bytes: [u8; 5] = [0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3333333333);
}
#[test]
fn test_014_array_bcd_to_u64() {
    let bytes: [u8; 5] = [0x11, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3333333311);
}
#[test]
fn test_015_array_bcd_to_u64() {
    let bytes: [u8; 5] = [0x11, 0x33, 0x33, 0x33, 0x55];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 5533333311);
}
#[test]
fn test_016_array_bcd_to_u64() {
    let bytes: [u8; 6] = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 333333333333);
}
#[test]
fn test_017_array_bcd_to_u64() {
    let bytes: [u8; 6] = [0x00, 0x33, 0x09, 0x33, 0x11, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 331133093300);
}
#[test]
fn test_018_array_bcd_to_u64() {
    let bytes: [u8; 6] = [0xDF, 0x33, 0xAB, 0x33, 0x11, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 331134113445);
}
#[test]
fn test_019_array_bcd_to_u64() {
    let bytes: [u8; 7] = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 33333333333333);
}
#[test]
fn test_020_array_bcd_to_u64() {
    let bytes: [u8; 7] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 33333333333311);
}
#[test]
fn test_021_array_bcd_to_u64() {
    let bytes: [u8; 7] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x22];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 22333333333311);
}
#[test]
fn test_022_array_bcd_to_u64() {
    let bytes: [u8; 8] = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3333333333333333);
}
#[test]
fn test_023_array_bcd_to_u64() {
    let bytes: [u8; 8] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 3333333333333311);
}
#[test]
fn test_024_array_bcd_to_u64() {
    let bytes: [u8; 8] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x22];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 2233333333333311);
}
#[test]
fn test_025_array_bcd_to_u64() {
    let bytes: [u8; 9] = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 333333333333333333);
}
#[test]
fn test_026_array_bcd_to_u64() {
    let bytes: [u8; 9] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 333333333333333311);
}
#[test]
fn test_027_array_bcd_to_u64() {
    let bytes: [u8; 9] = [0x11, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x22];
    let bcd: u64 = array_bcd_to_u64(&bytes);
    assert_eq!(bcd, 223333333333333311);
}
#[test]
fn test_001_array_24_to_int_32() {
    let bytes: [u8; 3] = [0xFF, 0xFF, 0xFF];
    let value: i32 = array_24_to_int_32(&bytes);
    println!("{}", value);
    assert_eq!(value, -1);
}
#[test]
fn test_002_array_24_to_int_32() {
    let bytes: [u8; 3] = [0xFF, 0xFF, 0x7F];
    let value: i32 = array_24_to_int_32(&bytes);
    println!("{}", value);
    assert_eq!(value, 8388607);
}
#[test]
fn test_003_array_24_to_int_32() {
    let bytes: [u8; 3] = [0x01, 0x00, 0x80];
    let value: i32 = array_24_to_int_32(&bytes);
    println!("{}", value);
    assert_eq!(value, -8388607);
}
#[test]
fn test_004_array_24_to_int_32() {
    let bytes: [u8; 3] = [0xED, 0xCB, 0x5D];
    let value: i32 = array_24_to_int_32(&bytes);
    println!("{}", value);
    assert_eq!(value, 6147053);
}
#[test]
fn test_001_array_48_to_int_64() {
    let bytes: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let value: i64 = array_48_to_int_64(&bytes);
    println!("{}", value);
    assert_eq!(value, -1);
}
#[test]
fn test_002_array_48_to_int_64() {
    let bytes: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
    let value: i64 = array_48_to_int_64(&bytes);
    println!("{}", value);
    assert_eq!(value, 140737488355327);
}
#[test]
fn test_003_array_48_to_int_64() {
    let bytes: [u8; 6] = [0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00];
    let value: i64 = array_48_to_int_64(&bytes);
    println!("{}", value);
    assert_eq!(value, 33554431);
}
#[test]
fn test_004_array_48_to_int_64() {
    let bytes: [u8; 6] = [0x00, 0xA7, 0x00, 0x00, 0x00, 0x00];
    let value: i64 = array_48_to_int_64(&bytes);
    println!("{}", value);
    assert_eq!(value, 42752);
}
#[test]
fn test_001_hex_string_to_byte_vector() {
    let hex_string: &str = "0x01-02-03-0A-FA";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), Some(vec![1, 2, 3, 10, 250]));
}
#[test]
fn test_002_hex_string_to_byte_vector() {
    let hex_string: &str = "0102030AFA";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), Some(vec![1, 2, 3, 10, 250]));
}
#[test]
fn test_003_hex_string_to_byte_vector() {
    let hex_string: &str = "01 02 03 0A FA";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), Some(vec![1, 2, 3, 10, 250]));
}
#[test]
fn test_004_hex_string_to_byte_vector() {
    let hex_string: &str = "01-02-03-0A-FI";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), None);
}
#[test]
fn test_005_hex_string_to_byte_vector() {
    let hex_string: &str = "0x01-02-03-0A-F";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), None);
}
#[test]
fn test_006_hex_string_to_byte_vector() {
    let hex_string: &str = "";
    let byte_vector: Option<Vec<u8>> = hex_string_to_byte_vector(hex_string);
    assert_eq!(byte_vector.map(|s| s), None);
}
#[test]
fn test_001_byte_to_hex_string() {
    let byte: u8 = 0x6A;  
    let result: String = byte_to_hex_string(&byte);
    assert_eq!(result, String::from("0x6A"));
}
#[test]
fn test_002_byte_to_hex_string() {
    let byte: u8 = 0x40;  
    let result: String = byte_to_hex_string(&byte);
    assert_eq!(result, String::from("0x40"));
}
#[test]
fn test_001_vec_bytes_to_hex_string() {
    let byte: [u8; 3] = [0x10, 0x20, 0x30];  
    let result: String = array_bytes_to_hex_string(&byte).unwrap_or_default();
    assert_eq!(result, String::from("0x10 0x20 0x30"));
}
#[test]
fn test_002_vec_bytes_to_hex_string() {
    let byte: [u8; 3] = [0x25, 0xAF, 0x7B];  
    let result: String = array_bytes_to_hex_string(&byte).unwrap_or_default();
    assert_eq!(result, String::from("0x25 0xAF 0x7B"));
}
#[test]
fn test_001_concatenate_str() {
    let string: Option<String> = None; 
    let string: Option<String> = string.concatenate_str("test");
    assert_eq!(string, Some(String::from("test")));
}
#[test]
fn test_002_concatenate_str() {
    let string: Option<String> = Some(String::from("test")); 
    let string: Option<String> = string.concatenate_str("test");
    assert_eq!(string, Some(String::from("test test")));
}