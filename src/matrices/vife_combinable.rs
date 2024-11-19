use crate::calculators::vib::*;

impl <'a> VifVife<'a> {
    pub fn get_vife_combinable(byte: u8) -> Self {   
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
            /* error */
            0x00 => {
                result.unit = "";
                result.description = "No error";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x01 => {
                result.unit = "";
                result.description = "Error: too many DIFEs";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x02 => {
                result.unit = "";
                result.description = "Error: storage number not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x03 => {
                result.unit = "";
                result.description = "Error: unit number not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x04 => {
                result.unit = "";
                result.description = "Error: tariff number not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x05 => {
                result.unit = "";
                result.description = "Error: function not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x06 => {
                result.unit = "";
                result.description = "Error: data class not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x07 => {
                result.unit = "";
                result.description = "Error: data size not implemented";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x08 => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x09 => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0A => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0B => {
                result.unit = "";
                result.description = "Error: too many VIFEs";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0C => {
                result.unit = "";
                result.description = "Error: illegal VIF group";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0D => {
                result.unit = "";
                result.description = "Error: illegal VIF exponent";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0E => {
                result.unit = "";
                result.description = "Error: VIF/DIF mismatch";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x0F => {
                result.unit = "";
                result.description = "Error: unimplemented action";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            /* reserved */
            0x10 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x11 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x12 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x13 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x14 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            /* error */
            0x15 => {
                result.unit = "";
                result.description = "Error: no data available";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x16 => {
                result.unit = "";
                result.description = "Error: data overflow";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x17 => {
                result.unit = "";
                result.description = "Error: data underflow";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x18 => {
                result.unit = "";
                result.description = "Error: data error";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x19 => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1A => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1B => {
                result.unit = "";
                result.description = "Error: reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x1C => {
                result.unit = "";
                result.description = "Error: premature end of record";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            /* Others */
            0x1D => {
                result.unit = "";
                result.description = "Standard conform data content";
                result.magnitude = None;
                result.data_type = Some(VibDataType::StandardConformData);
                return result;
            },
            0x1E => {
                result.unit = "";
                result.description = "Compact profile with register";
                result.magnitude = None;
                result.data_type = Some(VibDataType::CompactProfileWithRegister);
                return result;
            },
            0x1F => {
                result.unit = "";
                result.description = "Compact profile";
                result.magnitude = None;
                result.data_type = Some(VibDataType::CompactProfile);
                return result;
            },
            0x20 => {
                result.unit = "";
                result.description = "Per second";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x21 => {
                result.unit = "";
                result.description = "Per minute";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x22 => {
                result.unit = "";
                result.description = "Per hour";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x23 => {
                result.unit = "";
                result.description = "Per day";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x24 => {
                result.unit = "";
                result.description = "Per week";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x25 => {
                result.unit = "";
                result.description = "Per month";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x26 => {
                result.unit = "";
                result.description = "Per year";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x27 => {
                result.unit = "";
                result.description = "Per revolution/measurement";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x28 => {
                result.unit = "";
                result.description = "Increment per input pulse on imput channel number 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x29 => {
                result.unit = "";
                result.description = "Increment per input pulse on imput channel number 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2A => {
                result.unit = "";
                result.description = "Increment per output pulse on output channel number 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2B => {
                result.unit = "";
                result.description = "Increment per output pulse on output channel number 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2C => {
                result.unit = "";
                result.description = "Per litre";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2D => {
                result.unit = "";
                result.description = "Per \u{00B3}";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2E => {
                result.unit = "";
                result.description = "Per kg";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x2F => {
                result.unit = "";
                result.description = "Per K";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x30 => {
                result.unit = "";
                result.description = "Per kWh";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x31 => {
                result.unit = "";
                result.description = "Per GJ";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x32 => {
                result.unit = "";
                result.description = "Per kW";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x33 => {
                result.unit = "";
                result.description = "Per (K*l)";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x34 => {
                result.unit = "";
                result.description = "Per V";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x35 => {
                result.unit = "";
                result.description = "Per A";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x36 => {
                result.unit = "";
                result.description = "Multiplied by s";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x37 => {
                result.unit = "";
                result.description = "Multiplied by s/V";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x38 => {
                result.unit = "";
                result.description = "Multiplied by s/A";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x39 => {
                result.unit = "";
                result.description = "Start date/time";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3A => {
                result.unit = "";
                result.description = "VIF contains uncorrected unit or value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3B => {
                result.unit = "";
                result.description = "Accumulation only if positive contributions";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3C => {
                result.unit = "";
                result.description = "Accumulation of abs value only if negative contributions";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3D => {
                result.unit = "";
                result.description = "Reserved for alternative non-metric unit system";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3E => {
                result.unit = "";
                result.description = "Value at base conditions";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x3F => {
                result.unit = "";
                result.description = "OBIS-declaration";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x40 => {
                result.unit = "";
                result.description = "Lower limit value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x41 => {
                result.unit = "";
                result.description = "Number of exceeds of lower";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x42 => {
                result.unit = "";
                result.description = "Date/time of begin/first/lower";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x43 => {
                result.unit = "";
                result.description = "Date/time of end/first/lower";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x44 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x45 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x46 => {
                result.unit = "";
                result.description = "Date/time of begin/last/lower";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x47 => {
                result.unit = "";
                result.description = "Date/time of end/last/lower";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x48 => {
                result.unit = "";
                result.description = "Upper limit value";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x49 => {
                result.unit = "";
                result.description = "Number of exceeds of upper";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4A => {
                result.unit = "";
                result.description = "Date/time of begin/first/upper";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4B => {
                result.unit = "";
                result.description = "Date/time of end/first/upper";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4E => {
                result.unit = "";
                result.description = "Date/time of begin/last/upper";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x4F => {
                result.unit = "";
                result.description = "Date/time of end/last/upper";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x50 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 0, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x51 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 0, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x52 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 0, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x53 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 0, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x54 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 1, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x55 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 1, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x56 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 1, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x57 => {
                result.unit = "";
                result.description = "Duration of limit exceed 0, 1, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x58 => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 0, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x59 => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 0, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5A => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 0, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5B => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 0, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5C => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 1, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5D => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 1, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5E => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 1, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x5F => {
                result.unit = "";
                result.description = "Duration of limit exceed 1, 1, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x60 => {
                result.unit = "";
                result.description = "Duration of 0, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x61 => {
                result.unit = "";
                result.description = "Duration of 0, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x62 => {
                result.unit = "";
                result.description = "Duration of 0, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x63 => {
                result.unit = "";
                result.description = "Duration of 0, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x64 => {
                result.unit = "";
                result.description = "Duration of 1, 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x65 => {
                result.unit = "";
                result.description = "Duration of 1, 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x66 => {
                result.unit = "";
                result.description = "Duration of 1, 2";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x67 => {
                result.unit = "";
                result.description = "Duration of 1, 3";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x68 => {
                result.unit = "";
                result.description = "Value during lower limit exceed";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x69 => {
                result.unit = "";
                result.description = "Leakage values";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6A => {
                result.unit = "";
                result.description = "Date/time of 0 above 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6B => {
                result.unit = "";
                result.description = "Date/time of 0 above 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6C => {
                result.unit = "";
                result.description = "Value during upper limit exceed";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6D => {
                result.unit = "";
                result.description = "Overflow values";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6E => {
                result.unit = "";
                result.description = "Date/time of 1 above 0";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x6F => {
                result.unit = "";
                result.description = "Date/time of 1 above 1";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x70 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(0-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x71 => {
                result.description = "";
                result.magnitude = Some(1-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x72 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(2-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x73 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(3-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x74 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(4-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x75 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(5-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x76 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(6-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x77 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(7-6);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x78 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x79 => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7A => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7B => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7C => {
                result.extension = Some(byte);
                result.unit = "";
                result.description = "Extension combinable 0xFC";
                result.magnitude = None;
                result.data_type = None;
                return result;
            },
            0x7D => {
                result.unit = "";
                result.description = "";
                result.magnitude = Some(3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7E => {
                result.unit = "";
                result.description = "Future value";
                result.magnitude = None;
                result.data_type = None;
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