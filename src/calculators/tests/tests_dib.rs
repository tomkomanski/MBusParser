use super::super::dib::*;

// calculate_data
#[test]
fn test_001_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::NoData, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_002_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data8BitInteger, &[0x09]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(9.0));
}
#[test]
fn test_003_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data8BitInteger, &[0xF9]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-7.0));
}
#[test]
fn test_004_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data16BitInteger, &[0x67, 0x67]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(26471.0));
}
#[test]
fn test_005_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data16BitInteger, &[0x67, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-153.0));
}
#[test]
fn test_006_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data24BitInteger, &[0x10, 0x67, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(91920.0));
}
#[test]
fn test_007_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data24BitInteger, &[0x10, 0x67, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-39152.0));
}
#[test]
fn test_008_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data32BitInteger, &[0x10, 0x67, 0xFF, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(33515280.0));
}
#[test]
fn test_009_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data32BitInteger, &[0x10, 0x67, 0xFF, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-39152.0));
}
#[test]
fn test_010_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data32BitReal, &[0x01, 0x00, 0x00, 0x00]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1.4012984643248171E-45));
}
#[test]
fn test_011_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data32BitReal, &[0x01, 0x00, 0x00, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-1.7014120374287884E+38));
}
#[test]
fn test_012_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data48BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1103789817857.0));
}
#[test]
fn test_013_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data48BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-7692303204351.0));
}
#[test]
fn test_014_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data64BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9, 0x00, 0x01]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(72331376711434240.0));
}
#[test]
fn test_015_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data64BitInteger, &[0x01, 0x00, 0x00, 0xFF, 0x00, 0xF9, 0x00, 0xFF]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(-71783811364421632.0));
}
#[test]
fn test_016_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SelectionForReadout, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_017_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data2DigitBCD, &[0x12]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(12.0));
}
#[test]
fn test_018_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data4DigitBCD, &[0x12, 0x13]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(1312.0));
}
#[test]
fn test_019_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data6DigitBCD, &[0x12, 0x13, 0x14]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(141312.0));
}
#[test]
fn test_020_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data8DigitBCD, &[0x12, 0x13, 0x14, 0x15]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(15141312.0));
}
#[test]
fn test_021_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::DataVariableLength, &[0x12]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(18.0));
}
#[test]
fn test_022_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::Data12DigitBCD, &[0x12, 0x13, 0x14, 0x15, 0x16, 0x17]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, Some(171615141312.0));
}
#[test]
fn test_023_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionManufacturerSpecific, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_024_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_025_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionIdleFilter, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_026_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionReserved0x3F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_027_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionReserved0x4F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_028_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionReserved0x5F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_029_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionReserved0x6F, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}
#[test]
fn test_030_calculate_data() {
    let result: Result<Option<f64>, ParserError> = DibDataType::calculate_data(&DibDataType::SpecialFunctionGlobalReadout, &[]);
    let data: Option<f64> = result.unwrap();
    assert_eq!(data, None);
}

// calculate_dib
#[test]
fn test_01_calculate_dib() {
    let data: Vec<u8> = vec![0x01];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x01);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data8BitInteger);   
    assert_eq!(dib.data_length, 1);
}
#[test]
fn test_02_calculate_dib() {
    let data: Vec<u8> = vec![0x02];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x02);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data16BitInteger);   
    assert_eq!(dib.data_length, 2);
}
#[test]
fn test_03_calculate_dib() {
    let data: Vec<u8> = vec![0x03];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x03);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data24BitInteger);   
    assert_eq!(dib.data_length, 3);
}
#[test]
fn test_04_calculate_dib() {
    let data: Vec<u8> = vec![0x04];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x04);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_12_calculate_dib() {
    let data: Vec<u8> = vec![0x12];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x12);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Maximum));
    assert_eq!(dib.data_type, DibDataType::Data16BitInteger);   
    assert_eq!(dib.data_length, 2);
}
#[test]
fn test_14_calculate_dib() {
    let data: Vec<u8> = vec![0x14];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x14);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Maximum));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_22_calculate_dib() {
    let data: Vec<u8> = vec![0x22];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x22);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Minimum));
    assert_eq!(dib.data_type, DibDataType::Data16BitInteger);   
    assert_eq!(dib.data_length, 2);
}
#[test]
fn test_0f_calculate_dib() {
    let data: Vec<u8> = vec![0x0F];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x0F);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, None);
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, None);
    assert_eq!(dib.data_type, DibDataType::SpecialFunctionManufacturerSpecific);   
    assert_eq!(dib.data_length, 0);
}
#[test]
fn test_44_calculate_dib() {
    let data: Vec<u8> = vec![0x44];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x44);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(1));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_54_calculate_dib() {
    let data: Vec<u8> = vec![0x54];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x54);
    assert_eq!(dib.dife_bytes, vec![] as Vec<u8>);
    assert_eq!(dib.storage_number, Some(1));
    assert_eq!(dib.tariff, None);
    assert_eq!(dib.subunit, None);   
    assert_eq!(dib.function_field, Some(DibFunctionField::Maximum));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_00_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x00];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x00]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(0));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_10_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x10];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x10]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(1));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_20_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x20];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x20]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(2));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_40_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x40];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x40]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(0));
    assert_eq!(dib.subunit, Some(1));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_50_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x50];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x50]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(1));
    assert_eq!(dib.subunit, Some(1));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_60_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x60];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x60]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(2));
    assert_eq!(dib.subunit, Some(1));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_80_40_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0x80, 0x40];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0x80, 0x40]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(0));
    assert_eq!(dib.subunit, Some(2));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_84_c0_40_calculate_dib() {
    let data: Vec<u8> = vec![0x84, 0xC0, 0x40];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0x84);
    assert_eq!(dib.dife_bytes, vec![0xC0, 0x40]);
    assert_eq!(dib.storage_number, Some(0));
    assert_eq!(dib.tariff, Some(0));
    assert_eq!(dib.subunit, Some(3));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_c4_10_calculate_dib() {
    let data: Vec<u8> = vec![0xC4, 0x10];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0xC4);
    assert_eq!(dib.dife_bytes, vec![0x10]);
    assert_eq!(dib.storage_number, Some(1));
    assert_eq!(dib.tariff, Some(1));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_c4_20_calculate_dib() {
    let data: Vec<u8> = vec![0xC4, 0x20];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0xC4);
    assert_eq!(dib.dife_bytes, vec![0x20]);
    assert_eq!(dib.storage_number, Some(1));
    assert_eq!(dib.tariff, Some(2));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_c4_3b_calculate_dib() {
    let data: Vec<u8> = vec![0xC4, 0x3B];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0xC4);
    assert_eq!(dib.dife_bytes, vec![0x3B]);
    assert_eq!(dib.storage_number, Some(23));
    assert_eq!(dib.tariff, Some(3));
    assert_eq!(dib.subunit, Some(0));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}
#[test]
fn test_c4_80_40_calculate_dib() {
    let data: Vec<u8> = vec![0xC4, 0x80, 0x40];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Dib, ParserError> = Dib::new(&mut buffer);
    let dib: Dib = result.unwrap();
    assert_eq!(dib.dif_byte, 0xC4);
    assert_eq!(dib.dife_bytes, vec![0x80, 0x40]);
    assert_eq!(dib.storage_number, Some(1));
    assert_eq!(dib.tariff, Some(0));
    assert_eq!(dib.subunit, Some(2));   
    assert_eq!(dib.function_field, Some(DibFunctionField::Instantaneous));
    assert_eq!(dib.data_type, DibDataType::Data32BitInteger);   
    assert_eq!(dib.data_length, 4);
}