use std::collections::VecDeque;
use crate::matrices::errors::*;

#[derive(Debug, PartialEq)]
pub enum ExtendedLinkLayerType {
    I,
    II,
    III,
    IV,
    V,
    None,
}

impl ExtendedLinkLayerType {
    fn new(ci_field: u8) -> Self {  
        match ci_field {         
            0x8C => ExtendedLinkLayerType::I,
            0x8D => ExtendedLinkLayerType::II,
            0x8E => ExtendedLinkLayerType::III,  
            0x8F => ExtendedLinkLayerType::IV,
            0x86 => ExtendedLinkLayerType::V,
            _ => ExtendedLinkLayerType::None
        }
    }

    fn get_length(&self) -> u8 {
        match *self {
            ExtendedLinkLayerType::I => 2,
            ExtendedLinkLayerType::II => 8,
            ExtendedLinkLayerType::III => 10,
            ExtendedLinkLayerType::IV => 16,
            ExtendedLinkLayerType::V => 3,
            ExtendedLinkLayerType::None => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExtendedLinkLayer {
    pub extended_link_layer_type: ExtendedLinkLayerType,
    pub cc: Option<u8>,
    pub acc: Option<u8>,
    pub sn: Option<[u8; 4]>,
    pub payload_crc: Option<[u8; 2]>,
    pub m2: Option<[u8; 2]>,
    pub a2: Option<[u8; 6]>,
    pub ecl: Option<u8>,
    pub rtd: Option<[u8; 2]>,
    pub rxl: Option<u8>,
    pub length: Option<u8>,
}

impl Default for ExtendedLinkLayer {
    fn default() -> Self {
        ExtendedLinkLayer {
            extended_link_layer_type: ExtendedLinkLayerType::None,
            cc: None,
            acc: None,
            sn: None,
            payload_crc: None,
            m2: None,
            a2: None,
            ecl: None,
            rtd: None,
            rxl: None,
            length: None,
        }
    }
}

impl ExtendedLinkLayer {
    pub fn new(ci_field: u8, data: &mut VecDeque<u8>) -> Result<Self, ParserError> {  
        let extended_link_layer_type: ExtendedLinkLayerType = ExtendedLinkLayerType::new(ci_field);
        
        if data.len() < extended_link_layer_type.get_length() as usize {
            return Err(ParserError::ExtendedLinkLayerCalculatorError);
        }

        let mut extended_link_layer: ExtendedLinkLayer = ExtendedLinkLayer {..ExtendedLinkLayer::default() };
        let length: u8 = extended_link_layer_type.get_length();

        if extended_link_layer_type == ExtendedLinkLayerType::None {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.length = Some(length);
            return Ok(extended_link_layer);
        }
        else if extended_link_layer_type == ExtendedLinkLayerType::I {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.cc = data.pop_front();
            extended_link_layer.acc = data.pop_front();
            extended_link_layer.length = Some(length);
            
            return Ok(extended_link_layer);
        }
        else if extended_link_layer_type == ExtendedLinkLayerType::II {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.cc = data.pop_front();
            extended_link_layer.acc = data.pop_front();
            extended_link_layer.sn = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.payload_crc = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.length = Some(length);

            return Ok(extended_link_layer);
        }
        else if extended_link_layer_type == ExtendedLinkLayerType::III {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.cc = data.pop_front();
            extended_link_layer.acc = data.pop_front();
            extended_link_layer.m2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.a2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.length = Some(length);

            return Ok(extended_link_layer);
        }
        else if extended_link_layer_type == ExtendedLinkLayerType::IV {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.cc = data.pop_front();
            extended_link_layer.acc = data.pop_front();
            extended_link_layer.m2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.a2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.sn = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.payload_crc = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
            extended_link_layer.length = Some(length);

            return Ok(extended_link_layer);
        }
        else if extended_link_layer_type == ExtendedLinkLayerType::V {
            extended_link_layer.extended_link_layer_type = extended_link_layer_type;
            extended_link_layer.cc = data.pop_front();
            extended_link_layer.acc = data.pop_front();
            extended_link_layer.length = Some(length);

            let ecl: u8 = data.pop_front().unwrap();
            extended_link_layer.ecl = Some(ecl);

            if ((ecl & 0x01) >> 0) == 1 {
                if data.len() < 8 {
                    return Err(ParserError::ExtendedLinkLayerCalculatorError);
                }

                extended_link_layer.m2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
                extended_link_layer.a2 = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
                extended_link_layer.length = Some(extended_link_layer.length.as_ref().unwrap() + 8);
            }

            if ((ecl & 0x02) >> 1) == 1 {
                if data.len() < 4 {
                    return Err(ParserError::ExtendedLinkLayerCalculatorError);
                }

                extended_link_layer.sn = Some([data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap(), data.pop_front().unwrap()]);
                extended_link_layer.length = Some(extended_link_layer.length.as_ref().unwrap() + 4);

            }

            if ((ecl & 0x08) >> 1) == 1 {
                if data.len() < 2 {
                    return Err(ParserError::ExtendedLinkLayerCalculatorError);
                }

                extended_link_layer.rtd = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
                extended_link_layer.length = Some(extended_link_layer.length.as_ref().unwrap() + 2);
            }

            if ((ecl & 0x10) >> 1) == 1 {
                if data.len() < 1 {
                    return Err(ParserError::ExtendedLinkLayerCalculatorError);
                }

                extended_link_layer.rxl = data.pop_front();
                extended_link_layer.length = Some(extended_link_layer.length.as_ref().unwrap() + 1);
            }

            if ((ecl & 0x80) >> 7) == 1 {
                if data.len() < 2 {
                    return Err(ParserError::ExtendedLinkLayerCalculatorError);
                }

                extended_link_layer.payload_crc = Some([data.pop_front().unwrap(), data.pop_front().unwrap()]);
                extended_link_layer.length = Some(extended_link_layer.length.as_ref().unwrap() + 2);
            }

            return Ok(extended_link_layer);
        }
        else {
            return Err(ParserError::ExtendedLinkLayerCalculatorError);
        }
    }
}