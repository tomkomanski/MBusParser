use crate::calculators::vib::*;

// Based on iEM31, iEM32 and iEM33
impl <'a> VifVife<'a> {
    pub fn new_vife_schneider(byte: u8) -> Self {
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
                result.description = "Average current";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x01 => {
                result.unit = "";
                result.description = "L1 value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x02 => {
                result.unit = "";
                result.description = "L2 value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x03 => {
                result.unit = "";
                result.description = "L3 value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x04 => {
                result.unit = "";
                result.description = "L-N avg";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x05 => {
                result.unit = "";
                result.description = "L1-L2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x06 => {
                result.unit = "";
                result.description = "L2-L3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x07 => {
                result.unit = "";
                result.description = "L3-L1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x08 => {
                result.unit = "";
                result.description = "L-L avg";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x09 => {
                result.unit = "";
                result.description = "Export energy value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0A => {
                result.unit = "";
                result.description = "Power factor";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0B => {
                result.unit = "Hz";
                result.description = "Frequency";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0C => {
                result.unit = "";
                result.description = "Energy reset date and time";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0D => {
                result.unit = "";
                result.description = "Partial energy value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0E => {
                result.unit = "";
                result.description = "Input metering reset date and time";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0F => {
                result.unit = "";
                result.description = "Input metering accumulation";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x10 => {
                result.unit = "";
                result.description = "Active tariff (Energy active rate)";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x11 => {
                result.unit = "";
                result.description = "Tariff control mode";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x12 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x13 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x14 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x15 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x16 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x17 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x18 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x19 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1A => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x20 => {
                result.unit = "";
                result.description = "Meter operation timer";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x21 => {
                result.unit = "";
                result.description = "Number of phases";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x22 => {
                result.unit = "";
                result.description = "Number of wires";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x23 => {
                result.unit = "";
                result.description = "Power system configuration";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x24 => {
                result.unit = "";
                result.description = "Nominal frequency";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x25 => {
                result.unit = "";
                result.description = "Number of VTs";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x26 => {
                result.unit = "";
                result.description = "VT primary";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x27 => {
                result.unit = "";
                result.description = "VT secondary";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x28 => {
                result.unit = "";
                result.description = "Number of CTs";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x29 => {
                result.unit = "";
                result.description = "CT primary";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2A => {
                result.unit = "";
                result.description = "CT secondary";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2B => {
                result.unit = "";
                result.description = "VT connection type";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2C => {
                result.unit = "";
                result.description = "Energy pulse duration";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2D => {
                result.unit = "";
                result.description = "Digital output association with active energy pulsing";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2E => {
                result.unit = "";
                result.description = "Pulse weight";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2F => {
                result.unit = "";
                result.description = "Pulse constant";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x30 => {
                result.unit = "";
                result.description = "Digital input association";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x31 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x32 => {
                result.unit = "";
                result.description = "Digital input status";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x33 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x34 => {
                result.unit = "";
                result.description = "Overload alarm setup";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x35 => {
                result.unit = "";
                result.description = "Pickup setpoint";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x36 => {
                result.unit = "";
                result.description = "Digital output association with overload alarm";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x37 => {
                result.unit = "";
                result.description = "Activated status";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x38 => {
                result.unit = "";
                result.description = "Acknowledgment";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x39 => {
                result.unit = "";
                result.description = "Date and time of last alarm";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3A => {
                result.unit = "";
                result.description = "Value at last alarm";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x40 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x41 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x42 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x43 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x44 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x45 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x46 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x47 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x48 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x49 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4A => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x50 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x51 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x52 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x53 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x54 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x55 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x56 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x57 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x58 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x59 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5A => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x60 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x61 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x62 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x63 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x64 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x65 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x66 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x67 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x68 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x69 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6A => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x70 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x71 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x72 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x73 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x74 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x75 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x76 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x77 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x78 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x79 => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7A => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7B => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7C => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7D => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7E => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7F => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x00..=0xFF => {
                result.unit = "";
                result.description = "Not implemented in parser";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
        }
    }
}