use super::super::header::*;

#[test]
fn test_001_calculate_header() {
    let ci_field: u8 = 0x72;
    let mut buffer: VecDeque<u8> = [0x09, 0x05, 0x80, 0x98, 0x01, 0x06, 0x01, 0x02, 0x24, 0x10, 0x20, 0x05].into();
    let header: Result<Header, ParserError> = Header::new(ci_field, &mut buffer);
    assert_eq!(header.as_ref().unwrap().header_type, HeaderType::Long);
    assert_eq!(header.as_ref().unwrap().manufacturer, Some([0x01, 0x06]));
    assert_eq!(header.as_ref().unwrap().version, Some(0x01));
    assert_eq!(header.as_ref().unwrap().device_type, Some(0x02));
    assert_eq!(header.as_ref().unwrap().access_number, Some(0x24));
    assert_eq!(header.as_ref().unwrap().status, Some(0x10));
    assert_eq!(header.as_ref().unwrap().configuration, Some([0x20, 0x05]));
    assert_eq!(header.as_ref().unwrap().encryption, Some(EncryptionMethod::AesCbcIvNonZero));
}
#[test]
fn test_002_calculate_header() {
    let ci_field: u8 = 0x7A;
    let mut buffer: VecDeque<u8> = [0x2F, 0x00, 0x30, 0x25].into();
    let header: Result<Header, ParserError> = Header::new(ci_field, &mut buffer);
    assert_eq!(header.as_ref().unwrap().header_type, HeaderType::Short);
    assert_eq!(header.as_ref().unwrap().manufacturer, None);
    assert_eq!(header.as_ref().unwrap().version, None);
    assert_eq!(header.as_ref().unwrap().device_type, None);
    assert_eq!(header.as_ref().unwrap().access_number, Some(0x2F));
    assert_eq!(header.as_ref().unwrap().status, Some(0x00));
    assert_eq!(header.as_ref().unwrap().configuration, Some([0x30, 0x25]));
    assert_eq!(header.as_ref().unwrap().encryption, Some(EncryptionMethod::AesCbcIvNonZero));
}