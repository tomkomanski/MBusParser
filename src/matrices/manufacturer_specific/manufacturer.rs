#[derive(Debug, PartialEq)]
pub enum Manufacturer {
    Unknown,
    Abb,
    Schneider,
}

impl Manufacturer {
    pub fn get_manufacturer (manufacturer: &Option<[u8; 2]>) -> Manufacturer {
        match manufacturer {
            Some([0x42, 0x04]) => Manufacturer::Abb,
            Some([0xA3, 0x4C]) => Manufacturer::Schneider,
            _ => Manufacturer::Unknown,
        }
    }
}