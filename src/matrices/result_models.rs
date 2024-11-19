use core::fmt;
use serde::Serializer;
use serde_derive::Serialize;
use crate::calculators::header::*;
use crate::calculators::data_record::*;
use crate::tools::tools::*;

#[derive(Debug, PartialEq)]
pub enum DecryptionStatus {
    NotEncrypted,
    Encrypted,
    Decrypted,
}

impl fmt::Display for DecryptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_type: &str = match self {
            DecryptionStatus::NotEncrypted => "Not encrypted",
            DecryptionStatus::Encrypted => "Encrypted",
            DecryptionStatus::Decrypted => "Decrypted",
        };
        write!(f, "{}", data_type)
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ParserResult {
    #[serde(rename = "Datagram")]
    pub datagram: Option<Datagram>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
}

impl Default for ParserResult {
    fn default() -> Self {
        ParserResult {
            datagram: None,
            error: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Datagram {
    #[serde(rename = "Information")]
    pub information: Information,
    #[serde(rename = "Header")]
    pub header: Header,
    #[serde(rename = "Data records")]
    pub data_record: Vec<DataRecord>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Information {
    #[serde(rename = "C field", serialize_with = "serialize_c_field")]
    pub c_field: u8,
    #[serde(rename = "Primary address")]
    pub primary_address: Option<u8>,
    #[serde(rename = "CI field", serialize_with = "serialize_ci_field")]
    pub ci_field: u8,
    #[serde(rename = "Decryption status", serialize_with = "serialize_decryption_status")]
    pub decryption_status: DecryptionStatus,
}

fn serialize_c_field<S>(x: &u8, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            let string: String = byte_to_hex_string(x);
            s.serialize_str(&string)
        }

fn serialize_ci_field<S>(x: &u8, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            let string: String = byte_to_hex_string(x);
            s.serialize_str(&string)
        }

fn serialize_decryption_status<S>(x: &DecryptionStatus, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, { 
            let string: String =  x.to_string();
            s.serialize_str(&string)
        }