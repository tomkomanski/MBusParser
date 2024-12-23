use super::super::dib::*;

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