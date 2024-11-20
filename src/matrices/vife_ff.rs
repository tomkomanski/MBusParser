use crate::calculators::vib::*;

impl <'a> VifVife<'a> {
    pub fn new_vife_ff(byte: u8) -> Self {   
        let vife_byte_without_extension: u8 = byte & 0x7F;
        let mut extension: Option<u8> = None;
        if ((byte & 0x80) >> 7) == 1 {
            extension = Some(byte);
        }
        let mut result: VifVife = VifVife {
            byte: byte,
            extension: extension,
            unit: "",
            description: "",
            magnitude: None,
            data_type: None, 
        };

        match vife_byte_without_extension {
            0x00..=0xFF => {
                result.unit = "";
                result.description = "VIFEs and data of this block are manufacturer specific";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
        }
    }
}