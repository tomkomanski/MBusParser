use super::super::checksum::*;

#[test]
fn test_001_calculate_mbus_cs() {
    let frame: Vec<u8> = vec![0x08, 0x00, 0x72, 0x50, 0x00, 0x41, 0x00, 0x01, 0x06, 0x15, 0x07, 0x99, 0xE8, 0x00, 0x00, 0x0C, 0x78, 0x50, 0x00, 0x41, 0x00, 0x04, 0x6D, 0x11, 0xB2, 0xA9, 0x2C, 0x04, 0x13, 0x43, 0x00, 0x00, 0x00, 0x02, 0x3B, 0x00, 0x00, 0x44, 0x13, 0x43, 0x00, 0x00, 0x00, 0x42, 0x6C, 0xA1, 0x2C, 0x02, 0x27, 0x1F, 0x01, 0x03, 0xFD, 0x17, 0x0C, 0x03, 0x08, 0x04, 0xFF, 0x0A, 0x01, 0x04, 0x00, 0x00, 0x02, 0xFF, 0x0B, 0x00, 0x00, 0x03, 0xFF, 0x0C, 0x07, 0x00, 0xB5, 0x0F, 0x00, 0x04, 0x28, 0x12, 0xFF, 0x00, 0x01, 0x02, 0x01, 0x00];
    let cs: Result<u8, ParserError>  = calculate_mbus_cs(&frame);
    assert_eq!(cs.map(|s| s), Ok(0x28));
}
#[test]
fn test_002_calculate_mbus_cs() {
    let frame: Vec<u8> = vec![0x08, 0x03, 0x72, 0x15, 0x30, 0x83, 0x96, 0x01, 0x06, 0x01, 0x02, 0x64, 0x10, 0x00, 0x00, 0x0E, 0x03, 0x96, 0x43, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x83, 0x3C, 0x18, 0x04, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x83, 0xFC, 0x10, 0x15, 0x48, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6D, 0x00, 0x09, 0xA9, 0x23, 0x0F, 0x01, 0xFD, 0x7B, 0xE4];
    let cs: Result<u8, ParserError> = calculate_mbus_cs(&frame);
    assert_eq!(cs.map(|s| s), Ok(0xD8));
}
#[test]
fn test_003_calculate_mbus_cs() {
    let frame: Vec<u8> = vec![0x08, 0x01, 0x72, 0x40, 0x69, 0x00, 0x33, 0xE1, 0x0E, 0x03, 0x00, 0x77, 0x08, 0x30, 0x85, 0x2F, 0x2F, 0x03, 0xFD, 0x08, 0x06, 0xC0, 0x00, 0x04, 0xFD, 0x10, 0x00, 0x00, 0x00, 0x00, 0x02, 0x65, 0xAB, 0x09, 0x12, 0x65, 0xBE, 0x09, 0x22, 0x65, 0x3A, 0x09, 0x01, 0xFD, 0x48, 0x25, 0x04, 0x6D, 0x12, 0x10, 0x7E, 0x28, 0x02, 0x7C, 0x01, 0x48, 0x31, 0x00, 0x0F, 0x01, 0xFD, 0x62, 0xB7, 0x01, 0xFD, 0x7B, 0xCA];
    let cs: Result<u8, ParserError> = calculate_mbus_cs(&frame);
    assert_eq!(cs.map(|s| s), Ok(0xE2));
}
#[test]
fn test_004_calculate_mbus_cs() {
    let frame: Vec<u8> = vec![];
    let cs: Result<u8, ParserError> = calculate_mbus_cs(&frame);
    assert_eq!(cs.map(|s| s), Err(ParserError::MbusChecksumCalculationError));
}
#[test]
fn test_001_calculate_wmbus_crc() {
    let frame: Vec<u8> = vec![0x08, 0x01, 0x72, 0x40, 0x69, 0x00, 0x33, 0xE1, 0x0E, 0x03];
    let cs: Option<Vec<u8>> = calculate_wmbus_crc(&frame);
    assert_eq!(cs.map(|s| s), Some(vec![0xE2, 0x53]));
}
#[test]
fn test_002_calculate_wmbus_crc() {
    let frame: Vec<u8> = vec![0x7A, 0x24, 0x10, 0x20, 0x05, 0xD5, 0xAB, 0x1B, 0x89, 0x40, 0x6F, 0xA0, 0x87, 0x6E, 0x3A, 0xA2];
    let cs: Option<Vec<u8>> = calculate_wmbus_crc(&frame);
    assert_eq!(cs.map(|s| s), Some(vec![0x29, 0x00]));
}
#[test]
fn test_003_calculate_wmbus_crc() {
    let frame: Vec<u8> = vec![0x97, 0x6A, 0xBD, 0xE8, 0x7E];
    let cs: Option<Vec<u8>> = calculate_wmbus_crc(&frame);
    assert_eq!(cs.map(|s| s), Some(vec![0xDC, 0x7E]));
}
#[test]
fn test_004_calculate_wmbus_crc() {
    let frame: Vec<u8> = vec![];
    let cs: Option<Vec<u8>> = calculate_wmbus_crc(&frame);
    assert_eq!(cs.map(|s| s), None);
}