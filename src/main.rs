use phf::phf_map;
use std::{
    char,
    fmt::Error,
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

static HEX_AS_DECIMAL: phf::Map<char, &'static str> = phf_map! {
    '1' => "1",
    '2' => "2",
    '3' => "3",
    '4' => "4",
    '5' => "5",
    '6' => "6",
    '7' => "7",
    '8' => "8",
    '9' => "9",
    'A' => "10",
    'B' => "11",
    'C' => "12",
    'D' => "13",
    'E' => "14",
    'F' => "15",
};

fn main() {
    match choose_conversion() {
        1 => println!("{}", hex_to_bin()),
        2 => println!("Hexadecimal"),
        _ => (),
    };
}

#[allow(unused_assignments)]
fn hex_to_bin() -> String {
    let mut hex_input = String::new();
    let mut binary = String::new();

    loop {
        print!("Hex number: ");

        stdout().flush().expect("Error flushing output.");
        match stdin().read_line(&mut hex_input) {
            Ok(_) => {
                match parse_hex(hex_input.clone()) {
                    Ok(hex_as_decimal) => {
                        binary = decimal_to_bin(hex_as_decimal);
                        break;
                    }
                    Err(_) => {
                        hex_input = String::from(""); // Reset input
                        continue;
                    }
                };
            }
            Err(error) => {
                println!("Error given: {error}, try again.\n");
                continue;
            }
        }
    }

    binary
}

fn decimal_to_bin(decimal_values: Vec<&'static str>) -> String {
    let mut binary = String::new();

    for num_str in &decimal_values {
        let mut num = num_str.parse::<u8>().unwrap();

        let mut value = 8;
        loop {
            if num / value == 1 || num == value {
                binary.push_str("1");
                num -= value;
            } else {
                binary.push_str("0");
            }

            if value == 1 {
                break;
            }
            value /= 2;
        }
        binary.push_str(" ");
    }

    binary
}

fn parse_hex(hex_input: String) -> Result<Vec<&'static str>, Error> {
    let characters: Vec<char> = hex_input
        .to_uppercase()
        .strip_suffix("\n")
        .expect("No \n")
        .chars()
        .collect();

    let mut chars_as_decimal: Vec<&'static str> = Vec::new();
    for character in characters {
        match HEX_AS_DECIMAL.get(&character) {
            Some(value) => {
                chars_as_decimal.push(value);
            }
            None => return Err(std::fmt::Error),
        }
    }

    Ok(chars_as_decimal)
}

fn choose_conversion() -> u8 {
    let mut input_str = String::new();

    loop {
        println!("What do you want to convert to:");
        println!("[1] Binary");
        println!("[2] Hexadecimal");
        print!("> ");

        stdout().flush().expect("Error flushing output.");
        match stdin().read_line(&mut input_str) {
            Ok(_) => {
                if !will_parse(input_str.trim().parse::<u8>()) {
                    println!("Not a valid input, try again.\n");
                    input_str = String::from(""); // Reset input
                    continue;
                }
                break;
            }
            Err(error) => {
                println!("Error given: {error}, try again.\n");
                continue;
            }
        };
    }

    let input = input_str.trim().parse::<u8>().unwrap();
    input
}

/// Checks if the string can be parsed to u8 and the parsed number is equal to
/// 1 or 2.
fn will_parse(parse_data: Result<u8, ParseIntError>) -> bool {
    match parse_data {
        Ok(num) => {
            if num == 1 || num == 2 {
                return true;
            }
            false
        }
        Err(_) => false,
    }
}
