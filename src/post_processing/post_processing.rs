use crate::matrices::device_type::DeviceType;
use crate::matrices::result_models::Datagram;
pub struct PostProcess;

impl PostProcess {
    pub fn process(datagram: &mut Datagram, key: &Vec<u8>) {
        Self::parse_wireless_mbus_data_container(datagram, key);
        Self::parse_ngp_manufacturer_specific_data(datagram, key);
        Self::humanize_device_type(datagram);
    }

    pub fn humanize_device_type(datagram: &mut Datagram,) {
        if datagram.header.device_type.is_some() {
            datagram.header.device_type_hum = Some(DeviceType::new(datagram.header.device_type.unwrap()).to_string());
        }
    }
}