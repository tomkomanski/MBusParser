use std::collections::VecDeque;
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use crate::calculators::data_record::*;
use crate::calculators::dib::*;
use crate::calculators::extended_link_layer::*;
use crate::calculators::header::*;
use crate::calculators::vib::*;
use crate::matrices::encryption_method::*;
use crate::matrices::errors::*;
use crate::matrices::result_models::*;
use crate::post_processing::post_processing::PostProcess;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

impl PostProcess {
    pub fn parse_wireless_mbus_data_container(datagram: &mut Datagram, key: &Vec<u8>) {
        if datagram.data_record.is_empty() {
            return;
        }

        let wireless_mbus_data_container_index: Option<usize> = 
            datagram.data_record
                .iter()
                .position(|x: &DataRecord|  x.dib.data_type == DibDataType::DataVariableLength &&
                                            x.vib.as_ref().is_some_and(|x: &Vib| x.data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::WirelessMbusDataContainer)));
        
        if wireless_mbus_data_container_index.is_none() {
            return;
        }

        let wireless_mbus_data_container_index: usize = wireless_mbus_data_container_index.unwrap();   
        let wireless_mbus_data_container_data: Option<&Vec<u8>> = 
            datagram.data_record
                .get_mut(wireless_mbus_data_container_index)
                .unwrap()
                .data
                .as_ref();

        if wireless_mbus_data_container_data.is_none() {
            return;
        }

        let wireless_mbus_data_container_data: &Vec<u8> = wireless_mbus_data_container_data.unwrap();
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(wireless_mbus_data_container_data.iter());
    
        if buffer.len() < 12 {
            return;
        }

        buffer.pop_front(); // remove Lvar length
        buffer.pop_front(); // remove L field
        let c_field: u8 = buffer.pop_front().unwrap();
        let manufacturer: [u8; 2] = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
        let identification_number: [u8; 4] = [buffer.pop_front().unwrap(), buffer.pop_front().unwrap(), buffer.pop_front().unwrap(), buffer.pop_front().unwrap()];
        let version: u8 = buffer.pop_front().unwrap();
        let device_type: u8 = buffer.pop_front().unwrap();
        let ci_field: u8 = buffer.pop_front().unwrap();
    
        let mut information: Information = Information {
            c_field: c_field,
            primary_address: None,
            ci_field: ci_field,
            decryption_status: DecryptionStatus::NotEncrypted,
        };

        let extended_link_layer: Result<ExtendedLinkLayer, ParserError> = ExtendedLinkLayer::new(ci_field, &mut buffer);

        if extended_link_layer.is_err() {
            return;
        }

        let extended_link_layer: ExtendedLinkLayer = extended_link_layer.unwrap();

        if extended_link_layer.extended_link_layer_type != ExtendedLinkLayerType::None {
            if buffer.len() < 1 {
                return;
            }
            information.ci_field = buffer.pop_front().unwrap();
        }

        let header: Result<Header, ParserError> = Header::new(information.ci_field, &mut buffer);

        if header.is_err() {
            return;
        }

        let mut header: Header = header.unwrap();
    
        if header.header_type == HeaderType::None {
            return;
        }
        else if header.header_type == HeaderType::Short {
            header.identification_number = Some(identification_number);
            header.manufacturer = Some(manufacturer);
            header.version = Some(version);
            header.device_type = Some(device_type);
        }
            
        if buffer.len() < 2 || buffer.len() % 16 != 0 {
            return;
        }

        if header.encryption.as_ref().is_some_and(|x| x == &EncryptionMethod::AesCbcIvNonZero) && buffer[0] != 0x2F && buffer[1] != 0x2F {      
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
    
                let mut encoded_data: Vec<u8> = buffer.drain(..).collect();
    
                let decoded_data: Result<&[u8], aes::cipher::block_padding::UnpadError> = Aes128CbcDec::new(key.as_slice().into(), &iv.into()).decrypt_padded_mut::<NoPadding>(&mut encoded_data);

                if decoded_data.is_ok() {
                    buffer.extend(decoded_data.unwrap());
                }
            }
        }

        if buffer.len() >= 2 && buffer[0] == 0x2F && buffer[1] == 0x2F {
            information.decryption_status = DecryptionStatus::Decrypted;
            let mut last_record_number: u8 = 
            datagram.data_record
                .iter()
                .max_by_key(|x: &&DataRecord| x.record_number)
                .unwrap()
                .record_number;
    
            while buffer.len() > 0 {
                let data_record: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &header.manufacturer);

                if data_record.is_err() {
                    return;
                }
                
                let data_record: Option<DataRecord> = data_record.unwrap();
    
                if data_record.is_some() {
                    let mut data_record: DataRecord = data_record.unwrap();
                    last_record_number += 1;
                    data_record.record_number = last_record_number;
                    datagram.data_record.push(data_record);
                }
            }
        }

        datagram.information.decryption_status = information.decryption_status;
        datagram.header.encryption = header.encryption;
    }
}