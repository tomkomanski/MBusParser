use super::super::vib::*;

#[test]
fn test_04_calculate_vib() {
    let data: Vec<u8> = vec![0x04];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0x04);
    assert_eq!(vib.vife_bytes, vec![] as Vec<u8>);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "Wh");
    assert_eq!(vib.description, "Energy");
    assert_eq!(vib.magnitude, Some(1));
}
#[test]
fn test_65_calculate_vib() {
    let data: Vec<u8> = vec![0x65];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0x65);
    assert_eq!(vib.vife_bytes, vec![] as Vec<u8>);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "\u{00B0}C");
    assert_eq!(vib.description, "External temperature");
    assert_eq!(vib.magnitude, Some(-2));
}
#[test]
fn test_6d_calculate_vib() {
    let data: Vec<u8> = vec![0x6D];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0x6D);
    assert_eq!(vib.vife_bytes, vec![] as Vec<u8>);
    assert_eq!(vib.data_type, Some(VibDataType::DataTypeFJIM));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "Time point (date & time)");
    assert_eq!(vib.magnitude, Some(0));
}
#[test]
fn test_7c_01_48_calculate_vib() {
    let data: Vec<u8> = vec![0x7C, 0x01, 0x48];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0x7C);
    assert_eq!(vib.vife_bytes, vec![0x01, 0x48]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "H");
    assert_eq!(vib.magnitude, Some(0));
}
#[test]
fn test_83_fc_10_calculate_vib() {
    let data: Vec<u8> = vec![0x83, 0xFc, 0x10];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0x83);
    assert_eq!(vib.vife_bytes, vec![0xFc, 0x10]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "Wh");
    assert_eq!(vib.description, "Energy Accumulation of abs value for both positive and negative contribution");
    assert_eq!(vib.magnitude, Some(0));
}
#[test]
fn test_fc_a2_73_04_6c_61_67_69_calculate_vib() {
    let data: Vec<u8> = vec![0xFC, 0xA2, 0x73, 0x04, 0x6C, 0x61, 0x67, 0x69];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0xFC);
    assert_eq!(vib.vife_bytes, vec![0xA2, 0x73, 0x04, 0x6C, 0x61, 0x67, 0x69]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "igal Per hour");
    assert_eq!(vib.magnitude, Some(-3));
}
#[test]
fn test_fd_08_calculate_vib() {
    let data: Vec<u8> = vec![0xFD, 0x08];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0xFD);
    assert_eq!(vib.vife_bytes, vec![0x08]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "Access number");
    assert_eq!(vib.magnitude, Some(0));
}
#[test]
fn test_fd_10_calculate_vib() {
    let data: Vec<u8> = vec![0xFD, 0x10];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0xFD);
    assert_eq!(vib.vife_bytes, vec![0x10]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "Customer location");
    assert_eq!(vib.magnitude, Some(0));
}
#[test]
fn test_fd_48_calculate_vib() {
    let data: Vec<u8> = vec![0xFD, 0x48];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0xFD);
    assert_eq!(vib.vife_bytes, vec![0x48]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "V");
    assert_eq!(vib.description, "Volts");
    assert_eq!(vib.magnitude, Some(-1));
}
#[test]
fn test_fd_62_calculate_vib() {
    let data: Vec<u8> = vec![0xFD, 0x62];
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());
    let result: Result<Vib, ParserError> = Vib::new(&mut buffer, &None);
    let vib: Vib = result.unwrap();
    assert_eq!(vib.vif_byte, 0xFD);
    assert_eq!(vib.vife_bytes, vec![0x62]);
    assert_eq!(vib.data_type, Some(VibDataType::Numeric));
    assert_eq!(vib.unit, "");
    assert_eq!(vib.description, "Control signal");
    assert_eq!(vib.magnitude, Some(0));
}