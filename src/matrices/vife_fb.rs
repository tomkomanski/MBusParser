use crate::calculators::vib::*;

impl <'a> VifVife<'a> {
    pub fn get_vife_fb(byte: u8) -> Self {   
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
            /* Energy */
            0x00 => {
                result.unit = "MWh";
                result.description = "Energy";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x01 => {
                result.unit = "MWh";
                result.description = "Energy";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reactive energy */
            0x02 => {
                result.unit = "kVARh";
                result.description = "Reactive energy";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x03 => {
                result.unit = "kVARh";
                result.description = "Reactive energy";
                result.magnitude = Some(1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Apparent energy */
            0x04 => {
                result.unit = "kVAh";
                result.description = "Apparent energy";
                result.magnitude = Some(1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x05 => {
                result.unit = "kVAh";
                result.description = "Apparent energy";
                result.magnitude = Some(1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x06 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x07 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Energy */
            0x08 => {
                result.unit = "GJ";
                result.description = "Energy";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x09 => {
                result.unit = "GJ";
                result.description = "Energy";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x0A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Energy */
            0x0C => {
                result.unit = "MCal";
                result.description = "Energy";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0D => {
                result.unit = "MCal";
                result.description = "Energy";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0E => {
                result.unit = "MCal";
                result.description = "Energy";
                result.magnitude = Some(2-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0F => {
                result.unit = "MCal";
                result.description = "Energy";
                result.magnitude = Some(3-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Volume */
            0x10 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(0+2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x11 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(1+2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x12 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x13 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reactive power */
            0x14 => {
                result.unit = "kVAR";
                result.description = "Reactive power";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x15 => {
                result.unit = "kVAR";
                result.description = "Reactive power";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x16 => {
                result.unit = "kVAR";
                result.description = "Reactive power";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x17 => {
                result.unit = "kVAR";
                result.description = "Reactive power";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Mass */
            0x18 => {
                result.unit = "t";
                result.description = "Mass";
                result.magnitude = Some(0+2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x19 => {
                result.unit = "t";
                result.description = "Mass";
                result.magnitude = Some(1+2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Relative humidity */
            0x1A => {
                result.unit = "%";
                result.description = "Relative humidity";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1B => {
                result.unit = "%";
                result.description = "Relative humidity";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x1C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Volume */
            0x20 => {
                result.unit = "feet\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x21 => {
                result.unit = "0,1 feet\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x22 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x23 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x24 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x25 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x26 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x27 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Power */
            0x28 => {
                result.unit = "MW";
                result.description = "Power";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x29 => {
                result.unit = "MW";
                result.description = "Power";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Phase */
            0x2A => {
                result.unit = "";
                result.description = "Phase U-U";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2B => {
                result.unit = "";
                result.description = "Phase U-I";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Frequency */
            0x2C => {
                result.unit = "Hz";
                result.description = "Frequency";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2D => {
                result.unit = "Hz";
                result.description = "Frequency";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2E => {
                result.unit = "Hz";
                result.description = "Frequency";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2F => {
                result.unit = "Hz";
                result.description = "Frequency";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Power */
            0x30 => {
                result.unit = "GJ/h";
                result.description = "Power";
                result.magnitude = Some(0-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x31 => {
                result.unit = "GJ/h";
                result.description = "Power";
                result.magnitude = Some(1-1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x32 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x33 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Apparent power */
            0x34 => {
                result.unit = "kVA";
                result.description = "Apparent power";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x35 => {
                result.unit = "kVA";
                result.description = "Apparent power";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x36 => {
                result.unit = "kVA";
                result.description = "Apparent power";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x37 => {
                result.unit = "kVA";
                result.description = "Apparent power";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Reserved */
            0x38 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x39 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x40 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x41 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x42 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x43 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x44 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x45 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x46 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x47 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x48 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x49 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x50 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x51 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x52 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x53 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x54 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x55 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x56 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x57 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x58 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x59 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x60 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x61 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x62 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x63 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x64 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x65 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x66 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x67 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x68 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x69 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x70 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x71 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x72 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x73 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Cold/warm temperature limit */
            0x74 => {
                result.unit = "\u{00B0}C";
                result.description = "Cold/warm temperature limit";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x75 => {
                result.unit = "\u{00B0}C";
                result.description = "Cold/warm temperature limit";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x76 => {
                result.unit = "\u{00B0}C";
                result.description = "Cold/warm temperature limit";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x77 => {
                result.unit = "\u{00B0}C";
                result.description = "Cold/warm temperature limit";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Cumulative max of active power */
            0x78 => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x79 => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7A => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7B => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7C => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(4-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7D => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(5-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7E => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(6-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7F => {
                result.unit = "W";
                result.description = "Cumulative max of active power";
                result.magnitude = Some(7-3);
                result.data_type = Some(VibDataType::Numeric);
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