use hex_to_binary::ui::{binary_to_hex, choose_conversion, hex_to_bin};

fn main() {
    match choose_conversion() {
        1 => hex_to_bin(),
        2 => binary_to_hex(),
        _ => (),
    };
}
