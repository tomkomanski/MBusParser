use crate::matrices::result_models::*;
use crate::matrices::errors::*;

pub fn parse(data: &Vec<u8>, key: &Vec<u8>) -> Result<Datagram, ParserError> {
    let _data: &Vec<u8> = data;
    let _key: &Vec<u8> = key;
    return Err(ParserError::LongFrameWmbusBError);
}