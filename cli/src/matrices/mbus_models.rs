use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ParserResult {
    #[serde(rename = "Datagram")]
    pub datagram: Option<Datagram>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Datagram {
    #[serde(rename = "Information")]
    pub information: Information,
    #[serde(rename = "Header")]
    pub header: Header,
    #[serde(rename = "Data records")]
    pub data_record: Vec<DataRecord>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Information {
    #[serde(rename = "C field")]
    pub c_field: String,
    #[serde(rename = "Primary address")]
    pub primary_address: Option<u8>,
    #[serde(rename = "CI field")]
    pub ci_field: String,
    #[serde(rename = "Decryption status")]
    pub decryption_status: String,
}

impl Information {
    pub fn display (&self) {
        println!("{:<25}{}", "C field:", self.c_field);
        if self.primary_address.is_some() {         
            println!("{:<25}{}", "Primary address:", self.primary_address.unwrap());
        }
        println!("{:<25}{}", "CI field:", self.ci_field);
        println!("{:<25}{}", "Decryption status:", self.decryption_status);
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Header {
    #[serde(rename = "Identification number")]
    pub identification_number: Option<u32>,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<u8>,
    #[serde(rename = "Device type")]
    pub device_type: Option<u8>,
    #[serde(rename = "Device type hum")]
    pub device_type_hum: Option<String>,
    #[serde(rename = "Access number")]
    pub access_number: Option<u8>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<String>,
}

impl Header {
    pub fn display (&self) {
        if self.identification_number.is_some() {
            println!("{:<25}{}", "Secondary address:", self.identification_number.unwrap());
        }
        else {
            println!("{}", "Secondary address:");
        }

        if self.manufacturer.is_some() {
            println!("{:<25}{}", "Manufacturer:", self.manufacturer.as_ref().unwrap());
        }
        else {
            println!("{}", "Manufacturer:");
        }

        if self.version.is_some() {
            println!("{:<25}{}", "Version:", self.version.unwrap());
        }
        else {
            println!("{}", "Version:");
        }

        if self.device_type_hum.is_some() {
            println!("{:<25}{}", "Device type:", self.device_type_hum.as_ref().unwrap());
        }
        else {
            println!("{}", "Device type:");
        }

        if self.access_number.is_some() {
            println!("{:<25}{}", "Access number:", self.access_number.unwrap());
        }
        else {
            println!("{}", "Access number:");
        }

        if self.status.is_some() {
            println!("{:<25}{}", "Status:", self.status.as_ref().unwrap());
        }
        else {
            println!("{}", "Status:");
        }

        if self.configuration.is_some() {
            println!("{:<25}{}", "Configuration:", self.configuration.as_ref().unwrap());
        }
        else {
            println!("{}", "Configuration:");
        }

        if self.encryption.is_some() {
            println!("{:<25}{}", "Encryption:", self.encryption.as_ref().unwrap());
        }
        else {
            println!("{}", "Encryption:");
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DataRecord {
    #[serde(rename = "Record number")]
    pub record_number: u8,
    #[serde(rename = "Dib")]
    pub dib: Dib,
    #[serde(rename = "Vib")]
    pub vib: Option<Vib>,
    #[serde(rename = "Data")]
    pub data: Option<String>,
    #[serde(rename = "Numeric value")]
    pub numeric_value: Option<f64>,
    #[serde(rename = "Text value")]
    pub text_value: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

impl DataRecord {
    pub fn display (&self) {
        println!("{}{}", "Record number: ", self.record_number.to_string());
        
        print!("{}{}", "Dif byte: ", self.dib.dif_byte);

        if self.dib.dife_bytes.is_some() {
            print!("{:<10}{}{}", "", "Dife bytes: ", self.dib.dife_bytes.as_ref().unwrap());
        }

        if self.vib.is_some() {
            print!("{:<10}{}{}", "", "Vif byte: ", self.vib.as_ref().unwrap().vif_byte);

            if self.vib.as_ref().unwrap().vife_bytes.is_some() {
                print!("{:<10}{}{}", "", "Vife bytes: ", self.vib.as_ref().unwrap().vife_bytes.as_ref().unwrap());
            }
        }

        if self.data.is_some() {
            print!("{:<10}{}{}", "", "Data: ", self.data.as_ref().unwrap());
        }

        println!();

        print!("{}{}", "Data type: ", self.dib.data_type);

        if self.dib.storage_number.is_some() {
            print!("{:<10}{}{}", "", "Storage number: ", self.dib.storage_number.unwrap());
        }

        if self.dib.tariff.is_some() {
            print!("{:<10}{}{}", "", "Tariff: ", self.dib.tariff.unwrap());
        }

        if self.dib.subunit.is_some() {
            print!("{:<10}{}{}", "", "Subunit: ", self.dib.subunit.unwrap());
        }

        println!();

        if self.numeric_value.is_some() {
            print!("{}{}", "Value: ", self.numeric_value.unwrap());
        }
        else if self.text_value.is_some() {
            print!("{}{}", "Value: ", self.text_value.as_ref().unwrap());
        }

        if self.vib.is_some() {
            if self.vib.as_ref().unwrap().unit.is_some() {
                print!("{:<1}{}", "", self.vib.as_ref().unwrap().unit.as_ref().unwrap());
            }

            if self.vib.as_ref().unwrap().description.is_some() {
                print!("{:<1}{}", "", self.vib.as_ref().unwrap().description.as_ref().unwrap());
            }
        }

        if self.dib.function_field.is_some() {
            print!("{:<1}{}", "", self.dib.function_field.as_ref().unwrap());
        } 

        if self.comment.is_some() {
            print!("{:<1}{}{}", "", "Comment: ", self.comment.as_ref().unwrap());
        }

        println!();
    }
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Dib {
    #[serde(rename = "Dif byte")]
    pub dif_byte: String,
    #[serde(rename = "Dife bytes")]
    pub dife_bytes: Option<String>,
    #[serde(rename = "Storage number")]
    pub storage_number: Option<i32>,
    #[serde(rename = "Tariff")]
    pub tariff: Option<i32>,
    #[serde(rename = "Subunit")]
    pub subunit: Option<i32>,
    #[serde(rename = "Function field")]
    pub function_field: Option<String>,
    #[serde(rename = "Data type")]
    pub data_type: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Vib {
    #[serde(rename = "Vif byte")]
    pub vif_byte: String,
    #[serde(rename = "Vife bytes")]
    pub vife_bytes: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}