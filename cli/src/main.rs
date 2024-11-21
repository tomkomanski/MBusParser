mod matrices {
    pub mod mbus_models;
}

use std::io;
use m_bus_parser::parser;
use matrices::mbus_models::Datagram;
use matrices::mbus_models::ParserResult;
use colored::Colorize;

fn main() {
    println!("{}", "----------------- F A R F O C E L   M - B U S   P A R S E R -----------------".bold().green());
    println!();

    loop {
        println!("{}", "Enter M-Bus frame (hex):".green());

        let mut frame_input: String = String::new();
        let input: Result<usize, io::Error> = io::stdin().read_line(&mut frame_input);
        if input.is_err() {
            println!("{}", input.unwrap_err().to_string().red());
            continue;
        }

        println!("{}", "Enter decryption key (hex) - optional:".green());

        let mut key_input: String = String::new();
        let input: Result<usize, io::Error> = io::stdin().read_line(&mut key_input);
        if input.is_err() {
            println!("{}", input.unwrap_err().to_string().red());
            continue;
        }

        let frame: &str = frame_input.as_str();
        let key: &str = key_input.as_str();

        let result: String = parser::parse_telegram(frame, key);

        let parsed_data: Result<ParserResult, serde_json::Error> = serde_json::from_str(&result);
        if parsed_data.is_err() {
            let parsed_data = parsed_data.unwrap_err();
            println!("{:?}", parsed_data.to_string().red());
            continue;
        }

        let parsed_data: ParserResult = parsed_data.unwrap();

        println!("{}", "----------------- F A R F O C E L   M - B U S   P A R S E R -----------------".bold().green());
        println!();

        if parsed_data.datagram.is_none() && parsed_data.error.is_some() {
            println!("{}", parsed_data.error.unwrap().red());
            println!();
        }
        else {
            let datagram: Datagram = parsed_data.datagram.unwrap();
            datagram.information.display();
            datagram.header.display();

            println!("{}", "");
            
            if datagram.data_record.is_empty() {

            }
            else {
                for n in datagram.data_record {
                    n.display();
                    println!();
                }
            }
        }

        println!("{}", "-----------------------------------------------------------------------------".bold().green());
    }
}