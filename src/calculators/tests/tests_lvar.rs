use super::super::lvar::*;

#[test]
fn test_001_calculate_lvar() {
    let lvar_first_byte: u8 = 0x0B;
    let data: Vec<u8> = vec![0x32, 0x72, 0x65, 0x74, 0x73, 0x61, 0x4D, 0x73, 0x75, 0x42, 0x4D];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0x0B, 0x32, 0x72, 0x65, 0x74, 0x73, 0x61, 0x4D, 0x73, 0x75, 0x42, 0x4D]);
    assert_eq!(result.numeric_value, None);
    assert_eq!(result.text_value, Some(String::from("MBusMaster2")));
}
#[test]
fn test_002_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC1;
    let data: Vec<u8> = vec![0x12];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC1, 0x12]);
    assert_eq!(result.numeric_value, Some(12.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_003_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC2;
    let data: Vec<u8> = vec![0x12, 0x34];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC2, 0x12, 0x34]);
    assert_eq!(result.numeric_value, Some(3412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_004_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC3;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC3, 0x12, 0x34, 0x56]);
    assert_eq!(result.numeric_value, Some(563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_005_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC4;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC4, 0x12, 0x34, 0x56, 0x78]);
    assert_eq!(result.numeric_value, Some(78563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_006_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC5;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC5, 0x12, 0x34, 0x56, 0x78, 0x11]);
    assert_eq!(result.numeric_value, Some(1178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_007_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC6;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC6, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22]);
    assert_eq!(result.numeric_value, Some(221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_008_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC7;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC7, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33]);
    assert_eq!(result.numeric_value, Some(33221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_009_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC8;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC8, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44]);
    assert_eq!(result.numeric_value, Some(4433221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_010_calculate_lvar() {
    let lvar_first_byte: u8 = 0xC9;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xC9, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44, 0x55]);
    assert_eq!(result.numeric_value, Some(554433221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_011_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD1;
    let data: Vec<u8> = vec![0x12];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD1, 0x12]);
    assert_eq!(result.numeric_value, Some(-12.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_012_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD2;
    let data: Vec<u8> = vec![0x12, 0x34];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD2, 0x12, 0x34]);
    assert_eq!(result.numeric_value, Some(-3412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_013_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD3;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD3, 0x12, 0x34, 0x56]);
    assert_eq!(result.numeric_value, Some(-563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_014_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD4;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD4, 0x12, 0x34, 0x56, 0x78]);
    assert_eq!(result.numeric_value, Some(-78563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_015_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD5;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD5, 0x12, 0x34, 0x56, 0x78, 0x11]);
    assert_eq!(result.numeric_value, Some(-1178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_016_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD6;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD6, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22]);
    assert_eq!(result.numeric_value, Some(-221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_017_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD7;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD7, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33]);
    assert_eq!(result.numeric_value, Some(-33221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_018_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD8;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD8, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44]);
    assert_eq!(result.numeric_value, Some(-4433221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_019_calculate_lvar() {
    let lvar_first_byte: u8 = 0xD9;
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xD9, 0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44, 0x55]);
    assert_eq!(result.numeric_value, Some(-554433221178563412.0));
    assert_eq!(result.text_value, None);
}
#[test]
fn test_020_calculate_lvar() {
    let lvar_first_byte: u8 = 0xE1;
    let data: Vec<u8> = vec![0xAA];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xE1, 0xAA]);
    assert_eq!(result.numeric_value, None);
    assert_eq!(result.text_value, Some(String::from("10101010")));
}
#[test]
fn test_021_calculate_lvar() {
    let lvar_first_byte: u8 = 0xE2;
    let data: Vec<u8> = vec![0xAA, 0xAA];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xE2, 0xAA, 0xAA]);
    assert_eq!(result.numeric_value, None);
    assert_eq!(result.text_value, Some(String::from("1010101010101010")));
}
#[test]
fn test_022_calculate_lvar() {
    let lvar_first_byte: u8 = 0xE4;
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xE4, 0xAA, 0xAA, 0xAA, 0xAA]);
    assert_eq!(result.numeric_value, None);
    assert_eq!(result.text_value, Some(String::from("10101010101010101010101010101010")));
}
#[test]
fn test_023_calculate_lvar() {
    let lvar_first_byte: u8 = 0xF6;
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Lvar, ParserError> = Lvar::new(lvar_first_byte, &mut buffer);
    let result: Lvar = result.unwrap();
    assert_eq!(result.data, vec![0xF6, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    assert_eq!(result.numeric_value, None);
    assert_eq!(result.text_value, Some(String::from("1010101010101010101010101010101010101010101010101010101010101010")));
}