#[cfg(test)]
mod tests_parser {  
    use std::fs;
    use m_bus_parser::parser::*;

    #[test]
    fn test_001_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/001_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_002_parse_gavazzi_elecricity() {
        let file: String = fs::read_to_string("./tests/test_files/002_gavazzi_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }  
    #[test]
    fn test_003_parse_apator_elecricity() {
        let file: String = fs::read_to_string("./tests/test_files/003_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_004_parse_apator_elecricity_encoded() {
        let file: String = fs::read_to_string("./tests/test_files/004_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_005_parse_apator_heat() {
        let file: String = fs::read_to_string("./tests/test_files/005_apator_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_006_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/006_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_007_parse_apator_elecricity_encoded() {
        let file: String = fs::read_to_string("./tests/test_files/007_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_008_parse_apator_elecricity_encoded() {
        let file: String = fs::read_to_string("./tests/test_files/008_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_009_parse_rel_gas() {
        let file: String = fs::read_to_string("./tests/test_files/009_rel_gas.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_010_parse_hyd_heat() {
        let file: String = fs::read_to_string("./tests/test_files/010_hyd_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_011_parse_nzr_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/011_nzr_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_012_parse_acw_water() {
        let file: String = fs::read_to_string("./tests/test_files/012_acw_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_013_parse_gwf_water() {
        let file: String = fs::read_to_string("./tests/test_files/013_gwf_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_014_parse_fin_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/014_fin_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_015_parse_gmc_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/015_gmc_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_016_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/016_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_017_parse_apator_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/017_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_018_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/018_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_019_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/019_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_020_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/020_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_021_parse_kam_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/021_kam_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_022_parse_apator_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/022_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_023_parse_kamstrup_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/023_kamstrup_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_024_parse_kamstrup_heat() {
        let file: String = fs::read_to_string("./tests/test_files/024_kamstrup_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_025_parse_kamstrup_heat() {
        let file: String = fs::read_to_string("./tests/test_files/025_kamstrup_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_026_parse_acw_heat() {
        let file: String = fs::read_to_string("./tests/test_files/026_acw_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_027_parse_apt_heat() {
        let file: String = fs::read_to_string("./tests/test_files/027_apt_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_028_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/028_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_029_parse_efe_heat() {
        let file: String = fs::read_to_string("./tests/test_files/029_efe_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_030_parse_bmeters_room_sensor() {
        let file: String = fs::read_to_string("./tests/test_files/030_bmeters_room_sensor.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_031_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/031_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_032_parse_emu_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/032_emu_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_033_parse_maddalena_water() {
        let file: String = fs::read_to_string("./tests/test_files/033_maddalena_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_034_parse_efe_heat_cooling() {
        let file: String = fs::read_to_string("./tests/test_files/034_efe_heat_cooling.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_035_parse_abb_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/035_abb_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_036_parse_schneider_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/036_schneider_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_037_parse_gavazzi_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/037_gavazzi_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_038_parse_apator_water() {
        let file: String = fs::read_to_string("./tests/test_files/038_apator_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_039_parse_apator_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/039_apator_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_040_parse_efe_water() {
        let file: String = fs::read_to_string("./tests/test_files/040_efe_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_041_parse_els_heat() {
        let file: String = fs::read_to_string("./tests/test_files/041_els_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_042_parse_els_water() {
        let file: String = fs::read_to_string("./tests/test_files/042_els_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_043_parse_emh_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/043_emh_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_044_parse_pad_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/044_pad_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_045_parse_edc_heat() {
        let file: String = fs::read_to_string("./tests/test_files/045_edc_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_046_parse_efe_heat() {
        let file: String = fs::read_to_string("./tests/test_files/046_efe_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_047_parse_abb_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/047_abb_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_048_parse_abb_electricity() {
        let file: String = fs::read_to_string("./tests/test_files/048_abb_electricity.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_049_parse_abb_heat() {
        let file: String = fs::read_to_string("./tests/test_files/049_abb_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_050_parse_acw_water() {
        let file: String = fs::read_to_string("./tests/test_files/050_acw_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_051_parse_acw_water() {
        let file: String = fs::read_to_string("./tests/test_files/051_acw_water.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_052_parse_slb_heat() {
        let file: String = fs::read_to_string("./tests/test_files/052_slb_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);
    }
    #[test]
    fn test_053_parse_amt_heat() {
        let file: String = fs::read_to_string("./tests/test_files/053_amt_heat.txt").unwrap();
        let mut lines: std::str::Lines = file.lines();
        let frame: &str = lines.next().unwrap();
        let key: &str = lines.next().unwrap();
        let datagram: &str = lines.next().unwrap();
        let parser_result: String = parse_telegram(&frame, &key);
        assert_eq!(parser_result, datagram);  
    }
}