use phf::phf_map;

pub mod conversion;
pub mod ui;

pub static HEX_AS_DECIMAL: phf::Map<char, &'static str> = phf_map! {
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
