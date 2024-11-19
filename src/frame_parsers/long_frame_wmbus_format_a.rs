use std::collections::VecDeque;
use std::iter;
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use crate::calculators::data_record::*;
use crate::calculators::extended_link_layer::*;
use crate::calculators::header::*;
use crate::matrices::errors::*;
use crate::matrices::encryption_method::*;
use crate::matrices::result_models::*;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn parse(data: &Vec<u8>, key: &Vec<u8>) -> Result<Datagram, ParserError> {
    let mut buffer: VecDeque<u8> = VecDeque::new();
    buffer.extend(data.iter());

    if buffer.len() < 13 {
        return Err(ParserError::LongFrameWmbusAError);
    }

    buffer.pop_front(); // remove L field
    let c_field: u8 = buffer.pop_front().unwrap();
    let manufacturer: [u8; 2] = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
    let identification_number: [u8; 4] = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap(), buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
    let version: u8 = buffer.pop_front().unwrap();
    let device_type: u8 = buffer.pop_front().unwrap();
    buffer.drain(..2); // remove CRC
    let ci_field: u8 = buffer.pop_front().unwrap();

    let mut information: Information = Information {
        c_field: c_field,
        primary_address: None,
        ci_field: ci_field,
        decryption_status: DecryptionStatus::NotEncrypted,
    };

    let extended_link_layer: Result<ExtendedLinkLayer, ParserError> = ExtendedLinkLayer::new(ci_field, &mut buffer);
    if extended_link_layer.is_err() {
        return Err(extended_link_layer.unwrap_err());
    }
    let extended_link_layer: ExtendedLinkLayer = extended_link_layer.unwrap();
    if extended_link_layer.extended_link_layer_type != ExtendedLinkLayerType::None {
        if buffer.len() < 1 {
            return Err(ParserError::LongFrameWmbusAError);
        }
        information.ci_field = buffer.pop_front().unwrap();
    }

    let header: Result<Header, ParserError> = Header::new(information.ci_field, &mut buffer);
    if header.is_err() {
        return Err(header.unwrap_err());
    }
    let mut header: Header = header.unwrap();

    if header.header_type == HeaderType::None {
        return Err(ParserError::LongFrameWmbusAError);
    }
    else if header.header_type == HeaderType::Short {
        header.identification_number = Some(identification_number);
        header.manufacturer = Some(manufacturer);
        header.version = Some(version);
        header.device_type = Some(device_type);
    }

    let mut aplication_layer: VecDeque<u8> = VecDeque::new();

    aplication_layer.extend(iter::repeat(0x00).take(header.get_length() as usize + 1)); // add zeroed ci field and header
    if extended_link_layer.extended_link_layer_type != ExtendedLinkLayerType::None {
        aplication_layer.extend(iter::repeat(0x00).take(extended_link_layer.length.unwrap() as usize + 1)); // add zeroed ci field and extended link layer
    }

    aplication_layer.extend(buffer);

    let mut potentially_encoded_data: VecDeque<u8> = VecDeque::new();
    while aplication_layer.len() >= 18 {
        potentially_encoded_data.extend(aplication_layer.drain(..16).collect::<Vec<u8>>());
        aplication_layer.drain(..2); // remove crc
    }
    if aplication_layer.len() > 2 {
        aplication_layer.drain(aplication_layer.len() - 2..); // remove crc
        potentially_encoded_data.extend(aplication_layer.drain(..).collect::<Vec<u8>>());
    }

    potentially_encoded_data.drain(..header.get_length() as usize + 1); // remove zereoed ci field and header
    if extended_link_layer.extended_link_layer_type != ExtendedLinkLayerType::None {
        potentially_encoded_data.drain(..extended_link_layer.length.unwrap() as usize + 1); // remove zereoed ci field and extended link layer
    }

    if potentially_encoded_data.len() < 2 || potentially_encoded_data.len() % 16 != 0 {
        return Err(ParserError::LongFrameWmbusAError);
    }

    let mut buffer: VecDeque<u8> = VecDeque::new();

    if header.encryption.as_ref().is_some_and(|x: &EncryptionMethod| x == &EncryptionMethod::AesCbcIvNonZero) && potentially_encoded_data[0] != 0x2F && potentially_encoded_data[1] != 0x2F {      
        information.decryption_status = DecryptionStatus::Encrypted;
        
        if key.len() == 16 {
            let mut iv: [u8; 16] = [0; 16];
            iv[0] = manufacturer[0];
            iv[1] = manufacturer[1];
            iv[2] = identification_number[0];
            iv[3] = identification_number[1];
            iv[4] = identification_number[2];
            iv[5] = identification_number[3];
            iv[6] = version;
            iv[7] = device_type;
            for n in 8..16 {
                iv[n] = header.access_number.unwrap();
            }

            let mut encoded_data: Vec<u8> = potentially_encoded_data.drain(..).collect();

            let decoded_data: Result<&[u8], aes::cipher::block_padding::UnpadError> = Aes128CbcDec::new(key.as_slice().into(), &iv.into()).decrypt_padded_mut::<NoPadding>(&mut encoded_data);
            if decoded_data.is_ok() {
                buffer.extend(decoded_data.unwrap());
            }
        }
    }

    let mut data_records: Vec<DataRecord> = Vec::new();

    if buffer.len() >= 2 && buffer[0] == 0x2F && buffer[1] == 0x2F {
        information.decryption_status = DecryptionStatus::Decrypted;

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
    }

    let datagram: Datagram = Datagram {
        information: information,
        header: header,
        data_record : data_records,
    };

   return Ok(datagram);
}
