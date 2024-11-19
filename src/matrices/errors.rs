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
    VibCalculatorError,
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
            ParserError::VibCalculatorError => "Vib calculator error",
        };
        write!(f, "{}", system_error)
    }
}