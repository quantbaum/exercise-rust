use int_enum::IntEnum;
use std::fmt;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntEnum, Sequence)]
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

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


pub fn color_to_value(_color: ResistorColor) -> u32 {
    // unimplemented!("convert a color into a numerical representation")
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    // unimplemented!(
    //     "convert the value {} into a string representation of color",
    //     value
    // )
    match ResistorColor::from_int(value) {
        Ok(n) => n.to_string(),
        Err(err) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    all::<ResistorColor>().collect::<Vec<_>>()
}
