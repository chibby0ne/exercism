use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
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
    ResistorColor::int_value(_color)
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
