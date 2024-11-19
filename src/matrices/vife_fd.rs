use crate::calculators::vib::*;

impl <'a> VifVife<'a> {
    pub fn get_vife_fd(byte: u8) -> Self {   
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
            /* Credit */
            0x00 => {
                result.unit = "";
                result.description = "Credit";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x01 => {
                result.unit = "";
                result.description = "Credit";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x02 => {
                result.unit = "";
                result.description = "Credit";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x03 => {
                result.unit = "";
                result.description = "Credit";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Debit */
            0x04 => {
                result.unit = "";
                result.description = "Debit";
                result.magnitude = Some(0-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x05 => {
                result.unit = "";
                result.description = "Debit";
                result.magnitude = Some(1-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x06 => {
                result.unit = "";
                result.description = "Debit";
                result.magnitude = Some(2-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x07 => {
                result.unit = "";
                result.description = "Debit";
                result.magnitude = Some(3-3);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x08 => {
                result.unit = "";
                result.description = "Access number";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x09 => {
                result.unit = "";
                result.description = "Device type";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0A => {
                result.unit = "";
                result.description = "Manufacturer";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0B => {
                result.unit = "";
                result.description = "Parameter set identification";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0C => {
                result.unit = "";
                result.description = "Model version";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0D => {
                result.unit = "";
                result.description = "Hardware version number";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0E => {
                result.unit = "";
                result.description = "Metrology (firmware) version";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x0F => {
                result.unit = "";
                result.description = "Other software version number";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x10 => {
                result.unit = "";
                result.description = "Customer location";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x11 => {
                result.unit = "";
                result.description = "Customer";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x12 => {
                result.unit = "";
                result.description = "Access code user";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x13 => {
                result.unit = "";
                result.description = "Access code operator";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x14 => {
                result.unit = "";
                result.description = "Access code system operato";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x15 => {
                result.unit = "";
                result.description = "Access code developer";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x16 => {
                result.unit = "";
                result.description = "Password";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x17 => {
                result.unit = "";
                result.description = "Error flags";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x18 => {
                result.unit = "";
                result.description = "Error mask";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x19 => {
                result.unit = "";
                result.description = "Security key";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1A => {
                result.unit = "";
                result.description = "Digital output";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1B => {
                result.unit = "";
                result.description = "Digital input";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1C => {
                result.unit = "";
                result.description = "Baudrate";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1D => {
                result.unit = "";
                result.description = "Response delay time";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1E => {
                result.unit = "";
                result.description = "Retry";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x1F => {
                result.unit = "";
                result.description = "Remote control";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x20 => {
                result.unit = "";
                result.description = "First storage number";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x21 => {
                result.unit = "";
                result.description = "Last storage nr";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x22 => {
                result.unit = "";
                result.description = "Size of storage block";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x23 => {
                result.unit = "";
                result.description = "Descriptor for tariff and subunit";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Storage intervals */
            0x24 => {
                result.unit = "second";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x25 => {
                result.unit = "minute";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x26 => {
                result.unit = "hour";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x27 => {
                result.unit = "day";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x28 => {
                result.unit = "month";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x29 => {
                result.unit = "year";
                result.description = "Storage interval";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x2A => {
                result.unit = "";
                result.description = "Operator specific data";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2B => {
                result.unit = "";
                result.description = "Time point second";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Duration since last readout */
            0x2C => {
                result.unit = "second";
                result.description = "Duration since last readout";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2D => {
                result.unit = "minute";
                result.description = "Duration since last readout";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2E => {
                result.unit = "hour";
                result.description = "Duration since last readout";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x2F => {
                result.unit = "day";
                result.description = "Duration since last readout";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x30 => {
                result.unit = "";
                result.description = "Start date/time of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Duration of tariff */
            0x31 => {
                result.unit = "second";
                result.description = "Duration of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x32 => {
                result.unit = "minute";
                result.description = "Duration of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x33 => {
                result.unit = "hour";
                result.description = "Duration of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Period of tariff */
            0x34 => {
                result.unit = "second";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x35 => {
                result.unit = "minute";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x36 => {
                result.unit = "hour";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x37 => {
                result.unit = "day";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x38 => {
                result.unit = "month";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x39 => {
                result.unit = "year";
                result.description = "Period of tariff";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x3A => {
                result.unit = "";
                result.description = "Dimensionless";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3B => {
                result.unit = "";
                result.description = "Data container for wireless M-Bus protocol";
                result.magnitude = None;
                result.data_type = Some(VibDataType::WirelessMbusDataContainer);
                return result;
            },
            /* Period of nominal data transmissions */
            0x3C => {
                result.unit = "second";
                result.description = "Period of nominal data transmissions";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3D => {
                result.unit = "minute";
                result.description = "Period of nominal data transmissions";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3E => {
                result.unit = "hour";
                result.description = "Period of nominal data transmissions";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x3F => {
                result.unit = "day";
                result.description = "Period of nominal data transmissions";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Volts */
            0x40 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(0-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x41 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(1-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x42 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(2-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x43 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(3-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x44 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(4-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x45 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(5-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x46 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(6-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x47 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(7-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x48 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(8-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x49 => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(9-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4A => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(10-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4B => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(11-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4C => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(12-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4D => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(13-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4E => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(14-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x4F => {
                result.unit = "V";
                result.description = "Volts";
                result.magnitude = Some(15-9);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Ampers */
            0x50 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(0-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x51 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(1-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x52 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(2-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x53 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(3-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x54 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(4-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x55 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(5-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x56 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(6-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x57 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(7-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x58 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(8-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x59 => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(9-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5A => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(10-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5B => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(11-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5C => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(12-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5D => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(13-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5E => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(14-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x5F => {
                result.unit = "A";
                result.description = "Ampers";
                result.magnitude = Some(15-12);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x60 => {
                result.unit = "";
                result.description = "Reset counter";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x61 => {
                result.unit = "";
                result.description = "Cumulation counter";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x62 => {
                result.unit = "";
                result.description = "Control signal";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x63 => {
                result.unit = "";
                result.description = "Day of week";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x64 => {
                result.unit = "";
                result.description = "Week number";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x65 => {
                result.unit = "";
                result.description = "Time point of day change";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x66 => {
                result.unit = "";
                result.description = "State of parameter activation";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x67 => {
                result.unit = "";
                result.description = "Special supplier information";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Duration since last cumulation */
            0x68 => {
                result.unit = "hour";
                result.description = "Duration since last cumulation";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x69 => {
                result.unit = "day";
                result.description = "Duration since last cumulation";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6A => {
                result.unit = "month";
                result.description = "Duration since last cumulation";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6B => {
                result.unit = "year";
                result.description = "Duration since last cumulation";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Operation time battery */
            0x6C => {
                result.unit = "hour";
                result.description = "Operating time battery";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6D => {
                result.unit = "day";
                result.description = "Operating time battery";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6E => {
                result.unit = "month";
                result.description = "Operating time battery";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x6F => {
                result.unit = "year";
                result.description = "Operating time battery";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            /* Others */
            0x70 => {
                result.unit = "";
                result.description = "Date and time of battery change";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x71 => {
                result.unit = "dBm";
                result.description = "RF level unit";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x72 => {
                result.unit = "";
                result.description = "Daylight saving";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x73 => {
                result.unit = "";
                result.description = "Listening window management";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x74 => {
                result.unit = "";
                result.description = "Remaining battery life time (days)";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x75 => {
                result.unit = "";
                result.description = "Number times the meter was stopped";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x76 => {
                result.unit = "";
                result.description = "Data container for manufacturer specific protocol";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::ManufacturerSpecificDataContainer);
                return result;
            },
            /* Reserved */
            0x77 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x78 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x79 => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7A => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7B => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7C => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7D => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7E => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
                result.data_type = Some(VibDataType::Numeric);
                return result;
            },
            0x7F => {
                result.unit = "";
                result.description = "Reserved";
                result.magnitude = Some(0);
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