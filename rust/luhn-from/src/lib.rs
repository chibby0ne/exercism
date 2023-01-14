use std::any::{TypeId, Any};
use std::fmt::Display;

pub struct Luhn {
    value: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {

    }
}

const STRING: TypeId  = TypeId::of::<String>();
const STR: TypeId  = TypeId::of::<&str>();
const USIZE: TypeId = TypeId::of::<usize>();
const U8: TypeId = TypeId::of::<u8>();
const U16: TypeId = TypeId::of::<u16>();
const U32: TypeId = TypeId::of::<u32>();
const U64: TypeId = TypeId::of::<u64>();

impl<'a, T: Any + Display> From<&'a T> for Luhn {
    fn from(input: &'a T) -> Self {
        let val = match TypeId::of::<T>() {
            STRING | STR => input.to_string(),
            _ => String::from(""),

        };
        Self {
            value: val,
        }
    }
}


// /// Here is the example of how the From trait could be implemented
// /// for the &str type. Naturally, you can implement this trait
// /// by hand for the every other type presented in the test suite,
// /// but your solution will fail if a new type is presented.
// /// Perhaps there exists a better solution for this problem?
// impl<'a> From<&'a str> for Luhn {
//     fn from(input: &'a str) -> Self {
//         unimplemented!("From the given input '{}' create a new Luhn struct.", input);
//     }
// }
