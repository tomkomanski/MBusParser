use core::fmt;

#[derive(Debug, PartialEq)]
pub enum EncryptionMethod {
    None,
    Reserved1,
    DesCbcIvZero,
    DesCbcIvNonZero,
    Reserved0x04,
    AesCbcIvNonZero,
    Reserved0x06,
    Reserved0x07,
    Reserved0x08,
    Reserved0x09,
    Reserved0x0A,
    Reserved0x0B,
    Reserved0x0C,
    Reserved0x0D,
    Reserved0x0E,
    Reserved0x0F,
}

impl EncryptionMethod {
    pub fn new(byte: u8) -> EncryptionMethod {
        let byte: u8 = byte & 0xF;
        match byte {
            0x00 => EncryptionMethod::None,
            0x01 => EncryptionMethod::Reserved1,
            0x02 => EncryptionMethod::DesCbcIvZero,
            0x03 => EncryptionMethod::DesCbcIvNonZero,
            0x04 => EncryptionMethod::Reserved0x04,
            0x05 => EncryptionMethod::AesCbcIvNonZero,
            0x06 => EncryptionMethod::Reserved0x06,
            0x07 => EncryptionMethod::Reserved0x07,
            0x08 => EncryptionMethod::Reserved0x08,
            0x09 => EncryptionMethod::Reserved0x09,
            0x0A => EncryptionMethod::Reserved0x0A,
            0x0B => EncryptionMethod::Reserved0x0B,
            0x0C => EncryptionMethod::Reserved0x0C,
            0x0D => EncryptionMethod::Reserved0x0D,
            0x0E => EncryptionMethod::Reserved0x0E,
            _ => EncryptionMethod::Reserved0x0F,
        }
    }
}

impl fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encryption_method: &str = match self {
            EncryptionMethod::None => "None",
            EncryptionMethod::Reserved1 => "Reserved",
            EncryptionMethod::DesCbcIvZero => "DES CBC with zero IV",
            EncryptionMethod::DesCbcIvNonZero => "DES CBC with non zero IV",
            EncryptionMethod::Reserved0x04 => "Reserved 0x04",
            EncryptionMethod::AesCbcIvNonZero => "AES CBC with non zero IV",
            EncryptionMethod::Reserved0x06 => "Reserved 0x06",
            EncryptionMethod::Reserved0x07 => "Reserved 0x07",
            EncryptionMethod::Reserved0x08 => "Reserved 0x08",
            EncryptionMethod::Reserved0x09 => "Reserved 0x09",
            EncryptionMethod::Reserved0x0A => "Reserved 0x0A",
            EncryptionMethod::Reserved0x0B => "Reserved 0x0B",
            EncryptionMethod::Reserved0x0C => "Reserved 0x0C",
            EncryptionMethod::Reserved0x0D => "Reserved 0x0D",
            EncryptionMethod::Reserved0x0E => "Reserved 0x0E",
            EncryptionMethod::Reserved0x0F => "Reserved 0x0F",
        };
        write!(f, "{}", encryption_method)
    }
}