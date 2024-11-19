use std::panic;
use crate::calculators::data_record::*;
use crate::calculators::telegram_format::*;
use crate::frame_parsers::long_frame_mbus;
use crate::frame_parsers::long_frame_wmbus_format_a;
use crate::frame_parsers::long_frame_wmbus_format_b;
use crate::matrices::device_type::*;
use crate::matrices::errors::*;
use crate::matrices::result_models::*;
use crate::tools::tools::*;

pub fn parse_telegram(frame: &str, decryption_key: &str) -> String {

    panic::set_hook(Box::new(|_info: &panic::PanicHookInfo| {
    }));

    let result = panic::catch_unwind(|| { 
        let mut parser_result: ParserResult = ParserResult {..Default::default() };

        let data: Option<Vec<u8>> = hex_string_to_byte_vector(frame);
        if  data.is_none() {
            parser_result.error = Some(ParserError::ParserError.to_string());
        }
        let mut key: Vec<u8> = vec![];
        let key_optioned: Option<Vec<u8>> = hex_string_to_byte_vector(decryption_key);
        if key_optioned.is_some() {
            key = key_optioned.unwrap()
        }

        let data: Vec<u8> = data.unwrap();
        let telegram_format: Result<TelegramFormat, ParserError> = TelegramFormat::calculate_telegram_format(&data);
        
        if telegram_format.is_err() {
            parser_result.error = Some(telegram_format.unwrap_err().to_string());
        }
        else {
            let telegram_format: TelegramFormat = telegram_format.unwrap();
            let datagram_result: Result<Datagram, ParserError> = match telegram_format {
                TelegramFormat::LongFrameMBus => long_frame_mbus::parse(&data, &key),
                TelegramFormat::LongFrameWMBusFormatA => long_frame_wmbus_format_a::parse(&data, &key),
                TelegramFormat::LongFrameWMBusFormatB => long_frame_wmbus_format_b::parse(&data, &key),
            };

            if datagram_result.is_err() {
                parser_result.error = Some(datagram_result.unwrap_err().to_string());
            }
            else {
                let mut datagram: Datagram = datagram_result.unwrap();

                // Post-processing wireless M-Bus data container
                DataRecord::parse_wireless_mbus_data_container(&mut datagram, &key);

                // Post-processing manufacturer specific data
                DataRecord::parse_ngp_manufacturer_specific_data(&mut datagram, &key);

                // Post-processing humanize device type
                if datagram.header.device_type.is_some() {
                    datagram.header.device_type_hum = Some(DeviceType::get_type(datagram.header.device_type.unwrap()).to_string());
                }

                parser_result.datagram = Some(datagram);
            }
        }
        return serde_json::to_string(&parser_result).unwrap();
    });

    if result.is_ok() {
        return result.unwrap();
    }
    else {
        let mut result: ParserResult = ParserResult {..Default::default() };
        result.error = Some(ParserError::ParserError.to_string());
        return serde_json::to_string(&result).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_telegram() {
        let hex_string: &str = "684C4C6808FD722852178201060102A9000000046D040001010DFD3B2F2E4401062852178201027AA90020057403A8202A01FCEB3C912E7F17D3BDB69C9A94FB25A2E3F567AE79CDDF3D74E001FD71BD8216";
        let key: &str = "";
        let result: String = parse_telegram(&hex_string, &key);
        println!("{}", result);
    }
}