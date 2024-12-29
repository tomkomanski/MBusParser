use super::super::data_record::*;

#[test]
fn test_001_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::NoData, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_002_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data8BitInteger, &[0x09]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(9.0));
}
#[test]
fn test_003_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data8BitInteger, &[0xF9]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-7.0));
}
#[test]
fn test_004_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data16BitInteger, &[0x67, 0x67]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(26471.0));
}
#[test]
fn test_005_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data16BitInteger, &[0x67, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-153.0));
}
#[test]
fn test_006_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data24BitInteger, &[0x10, 0x67, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(91920.0));
}
#[test]
fn test_007_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data24BitInteger, &[0x10, 0x67, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-39152.0));
}
#[test]
fn test_008_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data32BitInteger, &[0x10, 0x67, 0xFF, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(33515280.0));
}
#[test]
fn test_009_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data32BitInteger, &[0x10, 0x67, 0xFF, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-39152.0));
}
#[test]
fn test_010_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data32BitReal, &[0x01, 0x00, 0x00, 0x00]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1.4012984643248171E-45));
}
#[test]
fn test_011_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data32BitReal, &[0x01, 0x00, 0x00, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-1.7014120374287884E+38));
}
#[test]
fn test_012_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data48BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1103789817857.0));
}
#[test]
fn test_013_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data48BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-7692303204351.0));
}
#[test]
fn test_014_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data64BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9, 0x00, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(72331376711434240.0));
}
#[test]
fn test_015_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data64BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9, 0x00, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-71783811364421632.0));
}
#[test]
fn test_016_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SelectionForReadout, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_017_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data2DigitBCD, &[0x12]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(12.0));
}
#[test]
fn test_018_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data4DigitBCD, &[0x12, 0x13]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1312.0));
}
#[test]
fn test_019_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data6DigitBCD, &[0x12, 0x13, 0x14]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(141312.0));
}
#[test]
fn test_020_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data8DigitBCD, &[0x12, 0x13, 0x14, 0x15]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(15141312.0));
}
#[test]
fn test_021_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::DataVariableLength, &[0x12]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(18.0));
}
#[test]
fn test_022_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::Data12DigitBCD, &[0x12, 0x13, 0x14, 0x15, 0x16, 0x17]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(171615141312.0));
}
#[test]
fn test_023_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionManufacturerSpecific, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_024_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_025_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionIdleFilter, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_026_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionReserved0x3F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_027_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionReserved0x4F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_028_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionReserved0x5F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_029_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionReserved0x6F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_030_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DataRecord::calculate_data_value(&DibDataType::SpecialFunctionGlobalReadout, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}

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