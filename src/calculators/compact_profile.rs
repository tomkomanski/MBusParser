use std::collections::VecDeque;
use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::format::ParseError;
use crate::calculators::data_record::*;
use crate::calculators::dib::*;
use crate::calculators::vib::*;
use crate::matrices::errors::*;

impl DataRecord {
    pub fn calculate_compact_profile(data_records: &mut Vec<DataRecord>) {
        if data_records.is_empty() {
            return;
        }

        let mut compact_profile_indexes: Vec<usize> = Vec::new();

        let mut index: usize = 0;
        while index != data_records.len() {
            if data_records[index].vib.as_ref().is_some_and(|x: &Vib| x.data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::CompactProfile)) {
                compact_profile_indexes.push(index);
            }
            index += 1;
        }

        for idx in compact_profile_indexes {
            if !data_records.get(idx).is_some_and(|x: &DataRecord| x.dib.storage_number.is_some_and(|x: u32| x > 0)) {
                continue;
            }
            let storage_number: u32 = data_records.get(idx).unwrap().dib.storage_number.unwrap();

            if !data_records.get(idx).is_some_and(|x: &DataRecord| x.dib.tariff.is_some()) {
                continue;
            }
            let tariff: u32 = data_records.get(idx).unwrap().dib.tariff.unwrap();

            if !data_records.get(idx).is_some_and(|x: &DataRecord| x.dib.subunit.is_some()) {
                continue;
            }
            let subunit: u32 = data_records.get(idx).unwrap().dib.subunit.unwrap();

            if !data_records.get(idx).is_some_and(|x: &DataRecord| x.data.is_some()) {
                continue;
            }
            let data: &Vec<u8> = data_records.get(idx).unwrap().data.as_ref().unwrap();

            if !data_records.get(idx).is_some_and(|x: &DataRecord| x.vib.is_some()) {
                continue;
            }
            let magnitude: Option<i8> = data_records.get(idx).unwrap().vib.as_ref().unwrap().magnitude;
            
            let mut buffer: VecDeque<u8> = VecDeque::new();
            buffer.extend(data.iter());
            
            if buffer.len() < 3 { // byte 1 - lvar length, byte 2 - spacing control byte, byte 3 - spacing value
                continue;
            }

            buffer.pop_front(); // remove lvar length
            let spacing_control_byte: u8 = buffer.pop_front().unwrap();
            let spacing_value: u8 = buffer.pop_front().unwrap();
            
            let increment_mode: u8 = spacing_control_byte >> 6;
            let spacing_unit: u8 = (spacing_control_byte & 0x30) >> 4;
            let element_data_type: DibDataType = DibDataType::new(spacing_control_byte & 0x0F);
            let element_length: u8 = DibDataType::get_data_length(&element_data_type);

            if (buffer.len() as u8) % element_length != 0 {
                continue;
            }

            let amount_of_elements: u8 = (buffer.len() as u8) / element_length;

            let base_time_record: Option<&DataRecord> = 
                data_records
                    .iter()
                    .find(|x: &&DataRecord| x.dib.storage_number.is_some_and(|x: u32| x == storage_number) &&
                                            x.dib.tariff.is_some_and(|x: u32| x == tariff) &&
                                            x.dib.subunit.is_some_and(|x: u32| x == subunit) &&
                                            x.vib.as_ref().is_some_and(|x: &Vib| x.data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::DataTypeFJIM)));
            
            if !base_time_record.is_some_and(|x: &DataRecord| x.text_value.is_some()) {
                continue;
            }
            let base_time: &str = base_time_record.as_ref().unwrap().text_value.as_ref().unwrap().as_str();   
            let base_time: Result<NaiveDateTime, ParseError> = NaiveDateTime::parse_from_str(base_time, "%Y-%m-%d %H:%M:%S");
            if base_time.is_err() {
                continue;
            }
            let base_time: NaiveDateTime = base_time.unwrap();

            let base_value_record: Option<&DataRecord> = 
                data_records
                    .iter()
                    .find(|x: &&DataRecord| x.dib.storage_number.is_some_and(|x: u32| x == storage_number) &&
                                            x.dib.tariff.is_some_and(|x: u32| x == tariff) &&
                                            x.dib.subunit.is_some_and(|x: u32| x == subunit) &&
                                            x.vib.as_ref().is_some_and(|x: &Vib| x.data_type.as_ref().is_some_and(|x: &VibDataType| x == &VibDataType::Numeric)));
        
            if !base_value_record.is_some_and(|x: &DataRecord| x.numeric_value.is_some()) {
                continue;
            }
            let mut base_value: f64 = base_value_record.as_ref().unwrap().numeric_value.unwrap();
 
            let mut text: String = String::new();
            for n in 1..=amount_of_elements {
                if (buffer.len() as u8) < element_length {
                    break;
                }
                
                let element_data_bytes: Vec<u8> = buffer.drain(..element_length as usize).collect();
                let element_value: Result<Option<f64>, ParserError> = DataRecord::calculate_data(&element_data_type, &element_data_bytes);
                if !element_value.as_ref().is_ok_and(|x: &Option<f64>| x.is_some()) {
                    break;
                }
                let mut element_value: f64 = element_value.unwrap().unwrap();

                if magnitude.is_none() {
                    break;
                }
                let magnitude: i8 = magnitude.unwrap();

                let base: f64 = 10.0;
                element_value = element_value * base.powf(magnitude as f64);
                
                // round if data is not i64 and multiplier <= 10 and multiplier >= 0.001
                if element_data_type != DibDataType::Data64BitInteger && magnitude <= 1 && magnitude >= -3 {
                    element_value = (element_value * 1000.0).round() / 1000.0;
                }

                // Calculate element value according to increment mode
                if increment_mode == 0 {
                    // Absolute value, do nothing
                }
                else if increment_mode == 1 {
                    element_value = base_value + element_value;
                }
                else if increment_mode == 2 {
                    element_value = base_value - element_value;
                }
                else if increment_mode == 3 {
                    base_value = base_value - element_value;
                    element_value = base_value;
                }
                // ---

                // Calculate element datetime according to spacing unit and spacing value
                let mut date_time: NaiveDateTime = base_time;  
                if spacing_value == 0 {
                    // Not spacing in time, do nothing
                }
                else if spacing_value >= 1 && spacing_value <= 250 {
                    if spacing_unit == 0 {
                        date_time = base_time + Duration::seconds((n * spacing_value) as i64);
                    }
                    else if spacing_unit == 1 {
                        date_time = base_time + Duration::minutes((n * spacing_value) as i64);
                    }
                    else if spacing_unit == 2 {
                        date_time = base_time + Duration::hours((n * spacing_value) as i64);
                    }
                    else if spacing_unit == 3 {
                        date_time = base_time + Duration::days((n * spacing_value) as i64);
                    }
                }
                else if spacing_value == 253 {
                    if spacing_unit == 3 {
                        date_time = base_time + Duration::days(15);
                    }
                }
                else if spacing_value == 254 {
                    if spacing_unit == 1 {
                        date_time = base_time.checked_add_months(chrono::Months::new((n * 6) as u32)).unwrap();
                    }
                    else if spacing_unit == 2 {
                        date_time = base_time.checked_add_months(chrono::Months::new((n * 3) as u32)).unwrap();
                    }
                    else if spacing_unit == 3 {
                        date_time = base_time.checked_add_months(chrono::Months::new((n * 1) as u32)).unwrap();
                    }
                }
                // ---

                text = (text + " " + "[" + date_time.format("%Y-%m-%d %H:%M:%S").to_string().as_str() + " " + format!("{}", element_value).as_str() + "]").trim().to_string();
            }

            data_records[idx].update_data_record_text_value(&text);  
        }

        return;
    }
}