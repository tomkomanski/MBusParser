use std::collections::VecDeque;
use crate::calculators::data_record::*;
use crate::calculators::dib::*;
use crate::calculators::vib::*;
use crate::matrices::errors::*;
use crate::matrices::result_models::*;
use crate::frame_parsers::long_frame_wmbus_format_a;
use crate::post_processing::post_processing::PostProcess;

impl PostProcess {
    pub fn parse_ngp_manufacturer_specific_data(datagram: &mut Datagram, key: &Vec<u8>) {
        if datagram.data_record.is_empty() {
            return;
        }

        let custom_vif_humidity_index: Option<usize> = 
            datagram.data_record
                .iter()
                .position(|x: &DataRecord| x.vib.as_ref().is_some_and(|x: &Vib| x.description == "H"));

        if custom_vif_humidity_index.is_some() {
            datagram.data_record
                .get_mut(custom_vif_humidity_index.unwrap())
                .unwrap()
                .vib
                .as_mut()
                .unwrap()
                .update_vib(Some("%"), Some("Humidity"));
        }

        let mut last_record_number: u8 = 
            datagram.data_record
                .iter()
                .max_by_key(|x: &&DataRecord| x.record_number)
                .unwrap()
                .record_number;

        let manufacturer_specific_data_record: Option<&DataRecord> = 
            datagram.data_record
                .iter()
                .find(|x: &&DataRecord| x.dib.data_type == DibDataType::SpecialFunctionManufacturerSpecific || 
                                        x.dib.data_type == DibDataType::SpecialFunctionManufacturerSpecificExtandedNextDatagram);

        if manufacturer_specific_data_record.is_none() {
            return;
        }

        let manufacturer_specific_data: &DataRecord = manufacturer_specific_data_record.unwrap();

        if manufacturer_specific_data.data.is_none() {
            return;
        }

        let data: &Vec<u8> = manufacturer_specific_data.data.as_ref().unwrap();
        let mut buffer: VecDeque<u8> = VecDeque::new();
        buffer.extend(data.iter());

        while buffer.len() > 0 {
            let new_record: Result<Option<DataRecord>, ParserError> = DataRecord::calculate_data_record(&mut buffer, &datagram.header.manufacturer);

            if new_record.is_err() {
                return;
            }

            let new_record: Option<DataRecord> = new_record.unwrap();

            if new_record.is_none() {
                continue;
            }

            let mut new_record: DataRecord = new_record.unwrap();

            //Rssi record
            if new_record.dib.dif_byte == 0x01 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x7B) {
                    
                new_record.vib
                    .as_mut()
                    .unwrap()
                    .update_vib(Some("dBm"), Some("Rssi"));

                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Control signal record
            else if new_record.dib.dif_byte == 0x01 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x62) {
                    
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Storage interval record
            else if new_record.dib.dif_byte == 0x02 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x25) {
            
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Storage interval record
            else if new_record.dib.dif_byte == 0x03 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x25) {
            
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Operationg time record
            else if new_record.dib.dif_byte == 0x04 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0x24) {
            
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Datetime record
            else if new_record.dib.dif_byte == 0x04 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0x6D) {
    
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Customer location record
            else if new_record.dib.dif_byte == 0x04 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x10) {
            
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Volts record
            else if new_record.dib.dif_byte == 0x03 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x48) {
                
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Error flags record
            else if new_record.dib.dif_byte == 0x04 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x17) {
                
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Control signal record
            else if new_record.dib.dif_byte == 0x03 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0xFD) &&
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes.len() == 1) && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vife_bytes[0] == 0x62) {
                
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Temperature difference record
            else if new_record.dib.dif_byte == 0x01 && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0x62)  {
                
                last_record_number += 1;
                new_record.record_number = last_record_number;
                datagram.data_record.push(new_record);
            }
            // Manufacturer specific lvar
            else if new_record.dib.dif_byte == 0x0D && 
                    new_record.vib.as_ref().is_some_and(|x: &Vib| x.vif_byte == 0x7F) && 
                    new_record.data.as_ref().is_some_and(|x: &Vec<u8>| x.len() >= 13) {
                
                let lvar_without_length: Vec<u8> = new_record.data.unwrap().drain(1..).collect();

                let datagram_result: Result<Datagram, ParserError> = long_frame_wmbus_format_a::parse(&lvar_without_length, key);

                if datagram_result.is_err() {
                    continue;
                }
                
                let datagram_result: Datagram = datagram_result.unwrap();

                for mut n in datagram_result.data_record {
                    last_record_number += 1;
                    n.record_number = last_record_number;
                    datagram.data_record.push(n);
                }

                datagram.information.decryption_status = datagram_result.information.decryption_status;
                datagram.header = datagram_result.header;
            }
        }
    } 
}