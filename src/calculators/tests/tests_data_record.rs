use super::super::data_record::*;

#[test]
fn test_001_calculate_data_record() {
    let data: Vec<u8> = vec![0x01, 0xFD, 0x62, 0xBF];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0xBF]);
    assert_eq!(numeric_value, Some(-65 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_002_calculate_data_record() {
    let data: Vec<u8> = vec![0x84, 0x00, 0x13, 0x84, 0x9E, 0x20, 0x02];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x84, 0x9E, 0x20, 0x02]);
    assert_eq!(numeric_value, Some(35692.164 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_003_calculate_data_record() {
    let data: Vec<u8> = vec![0x04, 0x24, 0x84, 0x06, 0x28, 0x05];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x84, 0x06, 0x28, 0x05]);
    assert_eq!(numeric_value, Some(86509188.0 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_004_calculate_data_record() {
    let data: Vec<u8> = vec![0x2E, 0x9F, 0x00, 0x65, 0xA2, 0x00, 0xFB, 0xA6, 0x00];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x65, 0xA2, 0x00, 0xFB, 0xA6, 0x00]);
    assert_eq!(numeric_value, Some(107610102650000.0 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_005_calculate_data_record() {
    let data: Vec<u8> = vec![0x04, 0x07, 0xA4, 0x41, 0x00, 0x00];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0xA4, 0x41, 0x00, 0x00]);
    assert_eq!(numeric_value, Some(168040000.0 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_006_calculate_data_record() {
    let data: Vec<u8> = vec![0x0B, 0x3B, 0x50, 0x49, 0x00];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x50, 0x49, 0x00]);
    assert_eq!(numeric_value, Some(4.95 as f64));
    assert_eq!(text_value, None);
    assert_eq!(comment, None);
}
#[test]
fn test_007_calculate_data_record() {
    let data: Vec<u8> = vec![0x06, 0x6D, 0x3B, 0x09, 0x2C, 0xF7, 0x21, 0x00];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x3B, 0x09, 0x2C, 0xF7, 0x21, 0x00]);
    assert_eq!(numeric_value, None);
    assert_eq!(text_value, Some(String::from("2023-01-23 12:09:59")));
    assert_eq!(comment, None);
}
#[test]
fn test_008_calculate_data_record() {
    let data: Vec<u8> = vec![0x0D, 0xFD, 0x0C, 0x0B, 0x32, 0x72, 0x65, 0x74, 0x73, 0x61, 0x4D, 0x73, 0x75, 0x42, 0x4D];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &None);
    let result: DataRecord = result.unwrap().unwrap();
    let data: Vec<u8> = result.data.unwrap();
    let numeric_value: Option<f64> = result.numeric_value;
    let text_value: Option<String> = result.text_value;
    let comment: Option<String> = result.comment;
    assert_eq!(data, vec![0x0B, 0x32, 0x72, 0x65, 0x74, 0x73, 0x61, 0x4D, 0x73, 0x75, 0x42, 0x4D]);
    assert_eq!(numeric_value, None);
    assert_eq!(text_value, Some(String::from("MBusMaster2")));
    assert_eq!(comment, None);
}