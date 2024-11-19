use std::collections::VecDeque;
use crate::calculators::data_record::*;
use crate::calculators::header::*;
use crate::matrices::errors::*;
use crate::matrices::result_models::*;

pub fn parse(data: &Vec<u8>, _key: &Vec<u8>) -> Result<Datagram, ParserError> {
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());

    if buffer.len() < 9 {
        return Err(ParserError::LongFrameParserError);
    }

    buffer.drain(..4); // remove 0x68 0xLL 0xLL 0x68
    buffer.pop_back(); // remove 0x16
    buffer.pop_back(); // remove CS

    let information: Information = Information {
        c_field: buffer.pop_front().unwrap(),
        primary_address: buffer.pop_front(),
        ci_field: buffer.pop_front().unwrap(),
        decryption_status: DecryptionStatus::NotEncrypted,
    };
 
    let header: Result<Header, ParserError> = Header::new(information.ci_field, &mut buffer);
    if header.is_err() {
        return Err(header.unwrap_err());
    }
    let header: Header = header.unwrap();

    let mut data_records: Vec<DataRecord> = Vec::new();
    let mut record_numer: u8 = 0;

    while buffer.len() > 0 {
        let data_record: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &header.manufacturer);
        if data_record.is_err() {
            return Err(data_record.unwrap_err());
        }
        let data_record: Option<DataRecord> = data_record.unwrap();

        if data_record.is_some() {
            let mut data_record: DataRecord = data_record.unwrap();
            record_numer += 1;
            data_record.record_number = record_numer;
            data_records.push(data_record);
        }
    }

    // Compact profile procedure
    DataRecord::calculate_compact_profile(&mut data_records);
    //

    let datagram: Datagram = Datagram {
        information: information,
        header: header,
        data_record : data_records,
    };

   return Ok(datagram);
}