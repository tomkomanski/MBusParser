use core::fmt;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ParserError {
    DataRecordCalculatorError,
    DibCalculatorError,
    ExtendedLinkLayerCalculatorError,
    HeaderCalculatorError,
    LongFrameParserError,
    LongFrameWmbusAError,
    LongFrameWmbusBError,
    LvarError,
    ParserError,
    TelegramFormatCalculatorError,
    TelegramFormatNotSupported,
    VibCalculatorError,

    MbusChecksumCalculationError,
    MbusInvalidChecksum(String),
    WmbusCrcCalculationError,
    WmbusInvalidCrc(String),
    MbusInvalidDatagramLength,
    WmbusInvalidDatagramLength,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let system_error = match self {
            ParserError::DataRecordCalculatorError => "Data record calculator error",
            ParserError::DibCalculatorError => "Dib calculator error",
            ParserError::ExtendedLinkLayerCalculatorError => "Extended link layer calculator error",
            ParserError::HeaderCalculatorError => "Header calculator error",
            ParserError::LongFrameParserError => "M-Bus long frame parser error",
            ParserError::LongFrameWmbusAError => "wM-Bus format A long frame parser error",
            ParserError::LongFrameWmbusBError => "wM-Bus format B long frame parser error",
            ParserError::LvarError => "Lvar error",
            ParserError::ParserError => "Parser error",
            ParserError::TelegramFormatCalculatorError => "Telegram format calculator error",
            ParserError::TelegramFormatNotSupported => "Telegram format not supported",
            ParserError::VibCalculatorError => "Vib calculator error",
            ParserError::MbusChecksumCalculationError => "M-Bus checksum calculation error",
            ParserError::MbusInvalidChecksum(message) => &("M-Bus invalid checksum: ".to_string() + message),
            ParserError::WmbusCrcCalculationError => "wM-Bus crc calculation error",
            ParserError::WmbusInvalidCrc(message) => &("wM-Bus invalid crc: ".to_string() + message),
            ParserError::MbusInvalidDatagramLength => "M-Bus invalid datagram length",
            ParserError::WmbusInvalidDatagramLength => "wM-Bus invalid datagram length",
        };
        write!(f, "{}", system_error)
    }
}