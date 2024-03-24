use core::fmt::{Debug, Display, Formatter, Result};

macro_rules! null_eq {
    ($type:ty) => {
        impl PartialEq<$type> for Null {
            fn eq(&self, _other: &$type) -> bool {
                false
            }
        }

        impl PartialEq<Null> for $type {
            fn eq(&self, _other: &Null) -> bool {
                false
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct Null {}

impl Null {
    pub fn new() -> Null {
        Null {}
    }
}

impl PartialEq for Null {
    fn eq(&self, _other: &Null) -> bool {
        true
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Null")
    }
}

null_eq!(bool);
null_eq!(i32);
null_eq!(f32);
null_eq!(String);