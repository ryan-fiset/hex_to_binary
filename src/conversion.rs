use std::fmt::Error;

use crate::HEX_AS_DECIMAL;

pub fn parse_hex(hex_input: String) -> Result<Vec<&'static str>, Error> {
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
            None => return Err(Error),
        }
    }

    Ok(chars_as_decimal)
}

pub fn decimal_to_bin(decimal_values: Vec<&'static str>) -> String {
    let mut binary = String::new();

    for num_str in decimal_values {
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
