use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence, IntEnum, Clone, Copy)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    // Converting a color into a numerical representation
    match _color {
        ResistorColor::Black => ResistorColor::Black.int_value(),
        ResistorColor::Blue => ResistorColor::Blue.int_value(),
        ResistorColor::Brown => ResistorColor::Brown.int_value(),
        ResistorColor::Green => ResistorColor::Green.int_value(),
        ResistorColor::Grey => ResistorColor::Grey.int_value(),
        ResistorColor::Orange => ResistorColor::Orange.int_value(),
        ResistorColor::Red => ResistorColor::Red.int_value(),
        ResistorColor::Violet => ResistorColor::Violet.int_value(),
        ResistorColor::White => ResistorColor::White.int_value(),
        ResistorColor::Yellow => ResistorColor::Yellow.int_value(),
    }
}

pub fn value_to_color_string(value: u32) -> String {
    // Converting value into a string representation of color.

    if let Ok(color) = ResistorColor::from_int(value) {
        format!("{:?}", color)
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // Return a list of all the colors ordered by resistance
    all::<ResistorColor>().collect::<Vec<_>>()
}
