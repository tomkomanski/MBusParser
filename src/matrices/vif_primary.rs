use crate::calculators::vib::*;

impl <'a> VifVife<'a> {
    pub fn get_vif_primary(byte: u8) -> Self {   
        let vif_byte_without_extension: u8 = byte & 0x7F;
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

        match vif_byte_without_extension {
            /* E000 0nnn    Energy Wh (0.001Wh to 10000Wh) */
            0x00 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x01 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x02 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x03 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x04 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(4-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x05 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(5-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x06 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(6-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x07 => {
                result.unit = "Wh";
                result.description = "Energy";
                result.magnitude = Some(7-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E000 1nnn    Energy  J (0.001kJ to 10000kJ) */
            0x08 => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x09 => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0A => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0B => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0C => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(4);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0D => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(5);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0E => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0F => {
                result.unit = "J";
                result.description = "Energy";
                result.magnitude = Some(7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E001 0nnn    Volume m3 (0.001l to 10000l) */
            0x10 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(0-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x11 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(1-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x12 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(2-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x13 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(3-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x14 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(4-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x15 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(5-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x16 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(6-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x17 => {
                result.unit = "m\u{00B3}";
                result.description = "Volume";
                result.magnitude = Some(7-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E001 1nnn    Mass kg (0.001kg to 10000kg) */
            0x18 => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x19 => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1A => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1B => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1C => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(4-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1D => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(5-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1E => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(6-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1F => {
                result.unit = "kg";
                result.description = "Mass";
                result.magnitude = Some(7-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E010 00nn    On Time s */
            0x20 => {
                result.unit = "second";
                result.description = "On time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x21 => {
                result.unit = "minute";
                result.description = "On time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x22 => {
                result.unit = "hour";
                result.description = "On time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x23 => {
                result.unit = "day";
                result.description = "On time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E010 01nn    Operating Time s */
            0x24 => {
                result.unit = "second";
                result.description = "Operating time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x25 => {
                result.unit = "minute";
                result.description = "Operating time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x26 => {
                result.unit = "hour";
                result.description = "Operating time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x27 => {
                result.unit = "day";
                result.description = "Operating time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E010 1nnn    Power W (0.001W to 10000W) */
            0x28 => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x29 => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2A => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2B => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2C => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(4-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2D => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(5-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2E => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(6-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2F => {
                result.unit = "W";
                result.description = "Power";
                result.magnitude = Some(7-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E011 0nnn    Power J/h (0.001kJ/h to 10000kJ/h) */
            0x30 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x31 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(1);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x32 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(2);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x33 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x34 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(4);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x35 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(5);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x36 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x37 => {
                result.unit = "J/h";
                result.description = "Power";
                result.magnitude = Some(7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E011 1nnn    Volume Flow m3/h (0.001l/h to 10000l/h) */
            0x38 => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(0-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x39 => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(1-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3A => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(2-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3B => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(3-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3C => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(4-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3D => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(5-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3E => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(6-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3F => {
                result.unit = "m\u{00B3}/h";
                result.description = "Volume flow";
                result.magnitude = Some(7-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E100 0nnn     Volume Flow ext.  m3/min (0.0001l/min to 1000l/min) */
            0x40 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(0-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x41 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(1-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x42 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(2-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x43 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(3-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x44 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(4-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x45 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(5-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x46 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(6-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x47 => {
                result.unit = "m\u{00B3}/min";
                result.description = "Volume flow";
                result.magnitude = Some(7-7);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E100 1nnn     Volume Flow ext.  m3/s (0.001ml/s to 10000ml/s) */
            0x48 => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(0-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x49 => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(1-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4A => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(2-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4B => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(3-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4C => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(4-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4D => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(5-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4E => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(6-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4F => {
                result.unit = "m\u{00B3}/s";
                result.description = "Volume flow";
                result.magnitude = Some(7-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E101 0nnn     Mass flow kg/h (0.001kg/h to 10000kg/h) */
            0x50 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x51 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x52 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x53 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x54 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(4-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x55 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(5-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x56 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(6-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x57 => {
                result.unit = "kg/h";
                result.description = "Mass flow";
                result.magnitude = Some(7-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E101 10nn     Flow Temperature °C (0.001°C to 1°C) */
            0x58 => {
                result.unit = "\u{00B0}C";
                result.description = "Flow temperature";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x59 => {
                result.unit = "\u{00B0}C";
                result.description = "Flow temperature";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5A => {
                result.unit = "\u{00B0}C";
                result.description = "Flow temperature";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5B => {
                result.unit = "\u{00B0}C";
                result.description = "Flow temperature";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E101 11nn     Return Temperature °C (0.001°C to 1°C) */
            0x5C => {
                result.unit = "\u{00B0}C";
                result.description = "Return temperature";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5D => {
                result.unit = "\u{00B0}C";
                result.description = "Return temperature";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5E => {
                result.unit = "\u{00B0}C";
                result.description = "Return temperature";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5F => {
                result.unit = "\u{00B0}C";
                result.description = "Return temperature";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E110 00nn    Temperature Difference  K   (mK to  K) */
            0x60 => {
                result.unit = "K";
                result.description = "Temperature difference";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x61 => {
                result.unit = "K";
                result.description = "Temperature difference";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x62 => {
                result.unit = "K";
                result.description = "Temperature difference";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x63 => {
                result.unit = "K";
                result.description = "Temperature difference";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E110 01nn     External Temperature °C (0.001°C to 1°C) */
            0x64 => {
                result.unit = "\u{00B0}C";
                result.description = "External temperature";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x65 => {
                result.unit = "\u{00B0}C";
                result.description = "External temperature";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x66 => {
                result.unit = "\u{00B0}C";
                result.description = "External temperature";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x67 => {
                result.unit = "\u{00B0}C";
                result.description = "External temperature";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E110 10nn     Pressure bar (1mbar to 1000mbar) */
            0x68 => {
                result.unit = "bar";
                result.description = "Pressure";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x69 => {
                result.unit = "bar";
                result.description = "Pressure";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6A => {
                result.unit = "bar";
                result.description = "Pressure";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6B => {
                result.unit = "bar";
                result.description = "Pressure";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E110 110n     Time Point */
            0x6C => {
                result.unit = "";
                result.description = "Time point (date)";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::DataTypeG);
                return result;
            },
            0x6D => {
                result.unit = "";
                result.description = "Time point (date & time)";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::DataTypeFJIM);
                return result;
            },
            /* E110 1110     Units for H.C.A. dimensionless */
            0x6E => {
                result.unit = "";
                result.description = "H.C.A.";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E110 1111     Reserved extension for 0xEF*/
            0x6F => {
                result.unit = "";
                result.description = "Extension 0xEF";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            /* E111 00nn     Averaging duration s */
            0x70 => {
                result.unit = "second";
                result.description = "Averaging duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x71 => {
                result.unit = "minute";
                result.description = "Averaging duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x72 => {
                result.unit = "hour";
                result.description = "Averaging duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x73 => {
                result.unit = "day";
                result.description = "Averaging duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* E111 01nn     Actuality duration s */
            0x74 => {
                result.unit = "second";
                result.description = "Actuality duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x75 => {
                result.unit = "minute";
                result.description = "Actuality duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x76 => {
                result.unit = "hours";
                result.description = "Actuality duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x77 => {
                result.unit = "day";
                result.description = "Actuality duration";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Other */
            0x78 => {
                result.unit = "";
                result.description = "Fabrication No";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x79 => {
                result.unit = "";
                result.description = "(Enhanced) Identification";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7A => {
                result.unit = "";
                result.description = "Bus Address";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Other */
            0x7B => {
                result.unit = "";
                result.description = "Extension 0xFB";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7C => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::CustomString);
                return result;
            },
            0x7D => {
                result.unit = "";
                result.description = "Extension 0xFD";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7E => {
                result.unit = "";
                result.description = "Any VIF";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::AnyVif);
                return result;
            },
            0x7F => {
                result.extension = Some(byte);
                result.unit = "";
                result.description = "VIFEs and data of this block are manufacturer specific";
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