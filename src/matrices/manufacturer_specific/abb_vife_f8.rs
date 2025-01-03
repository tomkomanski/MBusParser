use crate::calculators::vib::*;

// Based on B23 and B24
impl <'a> VifVife<'a> {
    pub fn new_vife_abb_f8(byte: u8) -> Self {
        let vife_byte_without_extension: u8 = byte & 0x7F;
        let mut extension: Option<u8> = None;
        if ((byte & 0x80) >> 7) == 1 {
            extension = Some(byte);
        };
        let mut result: VifVife = VifVife {
            byte: byte,
            extension: extension,
            unit: "",
            description: "",
            magnitude: None,
            data_type: None, 
        };

        match vife_byte_without_extension {
            0x00 => {
                result.unit = "";
                result.description = "0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x01 => {
                result.unit = "";
                result.description = "1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x02 => {
                result.unit = "";
                result.description = "2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x03 => {
                result.unit = "";
                result.description = "3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x04 => {
                result.unit = "";
                result.description = "4";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x05 => {
                result.unit = "";
                result.description = "5";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x06 => {
                result.unit = "";
                result.description = "6";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x07 => {
                result.unit = "";
                result.description = "7";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x08 => {
                result.unit = "";
                result.description = "8";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x09 => {
                result.unit = "";
                result.description = "9";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0A => {
                result.unit = "";
                result.description = "10";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0B => {
                result.unit = "";
                result.description = "11";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0C => {
                result.unit = "";
                result.description = "12";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0D => {
                result.unit = "";
                result.description = "13";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0E => {
                result.unit = "";
                result.description = "14";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0F => {
                result.unit = "";
                result.description = "15";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x10 => {
                result.unit = "";
                result.description = "16";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x11 => {
                result.unit = "";
                result.description = "17";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x12 => {
                result.unit = "";
                result.description = "18";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x13 => {
                result.unit = "";
                result.description = "19";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x14 => {
                result.unit = "";
                result.description = "20";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x15 => {
                result.unit = "";
                result.description = "21";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x16 => {
                result.unit = "";
                result.description = "22";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x17 => {
                result.unit = "";
                result.description = "23";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x18 => {
                result.unit = "";
                result.description = "24";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x19 => {
                result.unit = "";
                result.description = "25";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1A => {
                result.unit = "";
                result.description = "26";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1B => {
                result.unit = "";
                result.description = "27";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1C => {
                result.unit = "";
                result.description = "28";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1D => {
                result.unit = "";
                result.description = "29";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1E => {
                result.unit = "";
                result.description = "30";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1F => {
                result.unit = "";
                result.description = "31";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x20 => {
                result.unit = "";
                result.description = "32";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x21 => {
                result.unit = "";
                result.description = "33";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x22 => {
                result.unit = "";
                result.description = "34";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x23 => {
                result.unit = "";
                result.description = "35";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x24 => {
                result.unit = "";
                result.description = "36";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x25 => {
                result.unit = "";
                result.description = "37";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x26 => {
                result.unit = "";
                result.description = "38";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x27 => {
                result.unit = "";
                result.description = "39";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x28 => {
                result.unit = "";
                result.description = "40";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x29 => {
                result.unit = "";
                result.description = "41";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2A => {
                result.unit = "";
                result.description = "42";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2B => {
                result.unit = "";
                result.description = "43";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2C => {
                result.unit = "";
                result.description = "44";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2D => {
                result.unit = "";
                result.description = "45";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2E => {
                result.unit = "";
                result.description = "46";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2F => {
                result.unit = "";
                result.description = "47";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x30 => {
                result.unit = "";
                result.description = "48";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x31 => {
                result.unit = "";
                result.description = "49";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x32 => {
                result.unit = "";
                result.description = "50";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x33 => {
                result.unit = "";
                result.description = "51";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x34 => {
                result.unit = "";
                result.description = "52";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x35 => {
                result.unit = "";
                result.description = "53";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x36 => {
                result.unit = "";
                result.description = "54";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x37 => {
                result.unit = "";
                result.description = "55";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x38 => {
                result.unit = "";
                result.description = "56";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x39 => {
                result.unit = "";
                result.description = "57";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3A => {
                result.unit = "";
                result.description = "58";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3B => {
                result.unit = "";
                result.description = "59";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3C => {
                result.unit = "";
                result.description = "60";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3D => {
                result.unit = "";
                result.description = "61";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3E => {
                result.unit = "";
                result.description = "62";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3F => {
                result.unit = "";
                result.description = "63";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x40 => {
                result.unit = "";
                result.description = "64";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x41 => {
                result.unit = "";
                result.description = "65";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x42 => {
                result.unit = "";
                result.description = "66";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x43 => {
                result.unit = "";
                result.description = "67";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x44 => {
                result.unit = "";
                result.description = "68";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x45 => {
                result.unit = "";
                result.description = "69";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x46 => {
                result.unit = "";
                result.description = "70";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x47 => {
                result.unit = "";
                result.description = "71";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x48 => {
                result.unit = "";
                result.description = "72";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x49 => {
                result.unit = "";
                result.description = "73";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4A => {
                result.unit = "";
                result.description = "74";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4B => {
                result.unit = "";
                result.description = "75";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4C => {
                result.unit = "";
                result.description = "76";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4D => {
                result.unit = "";
                result.description = "77";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4E => {
                result.unit = "";
                result.description = "78";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4F => {
                result.unit = "";
                result.description = "79";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x50 => {
                result.unit = "";
                result.description = "80";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x51 => {
                result.unit = "";
                result.description = "81";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x52 => {
                result.unit = "";
                result.description = "82";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x53 => {
                result.unit = "";
                result.description = "83";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x54 => {
                result.unit = "";
                result.description = "84";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x55 => {
                result.unit = "";
                result.description = "85";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x56 => {
                result.unit = "";
                result.description = "86";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x57 => {
                result.unit = "";
                result.description = "87";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x58 => {
                result.unit = "";
                result.description = "88";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x59 => {
                result.unit = "";
                result.description = "89";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5A => {
                result.unit = "";
                result.description = "90";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5B => {
                result.unit = "";
                result.description = "91";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5C => {
                result.unit = "";
                result.description = "92";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5D => {
                result.unit = "";
                result.description = "93";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5E => {
                result.unit = "";
                result.description = "94";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5F => {
                result.unit = "";
                result.description = "95";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x60 => {
                result.unit = "";
                result.description = "96";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x61 => {
                result.unit = "";
                result.description = "97";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x62 => {
                result.unit = "";
                result.description = "98";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x63 => {
                result.unit = "";
                result.description = "99";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x64 => {
                result.unit = "";
                result.description = "100";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x65 => {
                result.unit = "";
                result.description = "101";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x66 => {
                result.unit = "";
                result.description = "102";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x67 => {
                result.unit = "";
                result.description = "103";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x68 => {
                result.unit = "";
                result.description = "104";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x69 => {
                result.unit = "";
                result.description = "105";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6A => {
                result.unit = "";
                result.description = "106";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6B => {
                result.unit = "";
                result.description = "107";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6C => {
                result.unit = "";
                result.description = "108";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6D => {
                result.unit = "";
                result.description = "109";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6E => {
                result.unit = "";
                result.description = "110";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6F => {
                result.unit = "";
                result.description = "111";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x70 => {
                result.unit = "";
                result.description = "112";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x71 => {
                result.unit = "";
                result.description = "113";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x72 => {
                result.unit = "";
                result.description = "114";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x73 => {
                result.unit = "";
                result.description = "115";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x74 => {
                result.unit = "";
                result.description = "116";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x75 => {
                result.unit = "";
                result.description = "117";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x76 => {
                result.unit = "";
                result.description = "118";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x77 => {
                result.unit = "";
                result.description = "119";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x78 => {
                result.unit = "";
                result.description = "120";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x79 => {
                result.unit = "";
                result.description = "121";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7A => {
                result.unit = "";
                result.description = "122";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7B => {
                result.unit = "";
                result.description = "123";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7C => {
                result.unit = "";
                result.description = "124";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7D => {
                result.unit = "";
                result.description = "125";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7E => {
                result.unit = "";
                result.description = "126";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7F => {
                result.unit = "";
                result.description = "127";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x80..=0xFF => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
        }
    }
}