use std::{fmt, ops::{Add, Sub, Mul, Div, Rem}, cmp::Ordering};
use crate::helper::types::Null;

// Macro
macro_rules! impl_arithmetic {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<ConnectorType> for ConnectorType {
            type Output = ConnectorType;

            fn $method(self, other: Self) -> Self::Output {
                match (self.clone(), other) {
                    (ConnectorType::Integer(lhs), ConnectorType::Integer(rhs)) => ConnectorType::Integer(lhs $operator rhs),
                    (ConnectorType::Float(lhs), ConnectorType::Float(rhs)) => ConnectorType::Float(lhs $operator rhs),
                    _ => panic!("Unsupported operation: {} for {:?}", stringify!($method), self),
                }
            }
        }

        impl $trait<i32> for ConnectorType {
            type Output = ConnectorType;

            fn $method(self, other: i32) -> Self::Output {
                match self {
                    ConnectorType::Integer(value) => ConnectorType::Integer(value $operator other),
                    _ => panic!("Unsupported operation: {} for {:?}", stringify!($method), self),
                }
            }
        }

        impl $trait<f32> for ConnectorType {
            type Output = ConnectorType;

            fn $method(self, other: f32) -> Self::Output {
                match self {
                    ConnectorType::Float(value) => ConnectorType::Float(value $operator other),
                    _ => panic!("Unsupported operation: {} for {:?}", stringify!($method), self),
                }
            }
        }
    };
}

macro_rules! impl_partial_eq {
    ($variant:ident, $trait:ty) => {
        impl PartialEq<$trait> for ConnectorType {
            fn eq(&self, other: &$trait) -> bool {
                match self {
                    ConnectorType::$variant(value) => value == other,
                    _ => false,
                }
            }
        }

        impl PartialEq<ConnectorType> for $trait {
            fn eq(&self, other: &ConnectorType) -> bool {
                match other {
                    ConnectorType::$variant(value) => value == self,
                    _ => false,
                }
            }
        }
    };
}

macro_rules! impl_ordering {
    ($variant:ident, $trait:ty) => {
        impl PartialOrd<$trait> for ConnectorType {
            fn partial_cmp(&self, other: &$trait) -> Option<Ordering> {
                match self {
                    ConnectorType::$variant(value) => value.partial_cmp(other),
                    _ => None,
                }
            }
        }

        impl PartialOrd<ConnectorType> for $trait {
            fn partial_cmp(&self, other: &ConnectorType) -> Option<Ordering> {
                match other {
                    ConnectorType::$variant(value) => self.partial_cmp(value),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_to_connectortype {
    ($from:ty, $conversion:expr) => {
        impl From<$from> for ConnectorType {
            fn from(value: $from) -> Self {
                $conversion(value)
            }
        }
    };
}

macro_rules! type_matcher {
    ($value:expr, $type:expr, $expressions:expr) => {{
        let index = match $type {
            ConnectorTypeId::INT => 0,
            ConnectorTypeId::FLOAT => 1,
            ConnectorTypeId::STRING => 2,
            ConnectorTypeId::BOOL => 3,
            ConnectorTypeId::NULL => return ConnectorType::Null(Null::new()),
            _ => panic!("Unsupported conversion from {:?} to {:?}", $value, $type),
        };
        $expressions[index].clone()
    }};
}

macro_rules! exp_conversion {
    ($type:ident, $variant:ident) => {
        match $variant {
            ConnectorType::Integer(value) => {
                type_matcher!(value, $type, [
                    ConnectorType::Integer(value.clone()),
                    ConnectorType::Float(value.clone() as f32),
                    ConnectorType::String(value.to_string()),
                    ConnectorType::Boolean(value.clone() != 0)
                ])
            },
            ConnectorType::Float(value) => {
                type_matcher!(value, $type, [
                    ConnectorType::Integer(value.clone() as i32),
                    ConnectorType::Float(value.clone()),
                    ConnectorType::String(value.to_string()),
                    ConnectorType::Boolean(value.clone() != 0.0)
                ])
            },
            ConnectorType::String(value) => {
                type_matcher!(value, $type, [
                    ConnectorType::Integer(value.parse::<i32>().unwrap_or(0)),
                    ConnectorType::Float(value.parse::<f32>().unwrap_or(0.0)),
                    ConnectorType::String(value.clone()),
                    ConnectorType::Boolean(value.parse::<bool>().unwrap_or(false))
                ])
            },
            ConnectorType::Boolean(value) => {
                type_matcher!(value, $type, [
                    ConnectorType::Integer(if value.clone() { 1 } else { 0 }),
                    ConnectorType::Float(if value.clone() { 1.0 } else { 0.0 }),
                    ConnectorType::String(value.to_string()),
                    ConnectorType::Boolean(value.clone())
                ])
            },
            ConnectorType::Null(_) => {
                type_matcher!(0, $type, [
                    ConnectorType::Integer(0),
                    ConnectorType::Float(0.0),
                    ConnectorType::String("".to_string()),
                    ConnectorType::Boolean(false)
                ])
            },
        }
    };
}

/// Represents different types of data that can be stored in a connector.
/// It makes for a more flexible and extensible system by allowing different types of data to be stored in a single variable.
/// Cross conversion between different types is also supported.
/// 
/// # Implementation:
/// - Implements arithmetic operations such as addition, subtraction, multiplication, division, and remainder for numeric types.
/// - Supports equality and comparison operations for all variants.
/// - Allows conversion from primitive types to `ConnectorType`.
/// - Supports conversion between different types.
/// 
/// # Examples:
/// ```
/// use crate::nodes::connector_manager::type::ConnectorType;
/// 
/// let int_type = ConnectorType::new(42);
/// let float_type = ConnectorType::new(3.14);
/// let string_type = ConnectorType::new("Hello, world!");
/// let boolean_type = ConnectorType::new(true);
/// let null_type = ConnectorType::new(crate::helper::types::Null::new());
/// ```
/// 
/// 
/// # Adding a new type:
/// To add more types, add a new variant to the enum `ConnectorType`, create a const identifier for it and implement the necessary traits.\
/// Macros are provided to simplify the process of implementing traits.\
/// 
/// ## New variant:
/// ```
/// // Update exp_conversion to include the new variant
/// // Also updating the existing conversions to include the new variant if necessary
/// macro_rules! type_matcher {
///     ($value:expr, $type:expr, $expressions:expr) => {{
///         let index = match $type {
///             ...,
///             ConnectorTypeId::BOOL => 3,
///             ConnectorTypeId::FOO => 4,
///             ...
///         };
///         $expressions[index].clone()
///     }};
/// }
/// macro_rules! exp_conversion {
///     ($type:ident, $variant:ident) => {
///         match $variant {
///             ..., // Existing variant conversions
///             ConnectorType::Foo(value) => {
///                 type_matcher!(value, $type, [
///                     ConnectorType::Integer(value.clone() as i32),
///                     ConnectorType::Float(value.clone() as f32),
///                     ConnectorType::String(value.clone().to_string()),
///                     ConnectorType::Boolean(false),
///                     ConnectorType::Foo(value.clone())
///                 ])
///             },
///         }
///     };
/// }
/// 
/// pub enum ConnectorType {
///    ...,
///    Foo(Bar)
/// }
/// 
/// ...
/// impl ConnectorTypeId {
///     ...
///     pub const FOO: i32 = 6;
/// }
/// 
/// impl ConnectorType {
///     ...
///     pub fn get_type(&self) -> i32 {
///         match self {
///             ...,
///             ConnectorType::Foo(_) => ConnectorTypeId::FOO,
///         }
///     }
/// }
/// 
/// impl Ord for ConnectorType {
///     fn cmp(&self, other: &Self) -> Ordering {
///         match (self, other) {
///             ...,
///             (ConnectorType::Foo(lhs), ConnectorType::Foo(rhs)) => lhs.cmp(rhs),
///             ...,
///         }
///     }
/// }
///     
/// impl fmt::Display for ConnectorType {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         match self {
///             ...,
///             ConnectorType::Foo(value) => write!(f, "{}", value),
///         }
///     }
/// }
/// 
/// impl_to_connectortype!(Foo, ConnectorType::Foo);
/// impl_partial_eq!(Foo, Foo);
/// 
/// // If foo is a primitive numeric type, ordering
/// impl_ordering!(Integer, i32);
/// impl_ordering!(Float, f32);
/// ```
/// 
/// ## Macros:
/// - `impl_arithmetic!`: Implements arithmetic operations for `ConnectorType` such as `+, -, *, /, %`
///     - Parameters:
///         - `$trait`: The trait to implement.
///         - `$method`: The method name.
///         - `$operator`: The operator to use.
///     - Usage: `impl_arithmetic!(Add, add, +);`
/// - `impl_partial_eq!`: Implements equality operations for `ConnectorType` such as `==, !=`
///     - Parameters:
///         - `$variant`: The variant to implement equality for.
///         - `$trait`: The trait to implement equality for.
///     - Usage: `impl_partial_eq!(Integer, i32);`
/// - `impl_ordering!`: Implements comparison operations for `ConnectorType` such as `>, <, >=, <=`
///     - Parameters:
///         - `$variant`: The variant to implement comparison for.
///         - `$trait`: The trait to implement comparison for.
///     - Usage: `impl_ordering!(Integer, i32);`
/// - `impl_to_connectortype!`: Implements conversion from primitive types to `ConnectorType`.
///     - Parameters:
///         - `$from`: The primitive type to convert from.
///         - `$conversion`: The conversion function.
///     - Usage: `impl_to_connectortype!(i32, ConnectorType::Integer);`
/// ```
///             fn foo<T>(value: T) -> ConnectorType
///             where
///                 T: Into<ConnectorType>
///             {
///                value.into()
///             }
/// ```
/// - `type_matcher`: Matches the type to the corresponding index and returns the expression at that index.
///     - Parameters:
///         - `$value`: The value to match.
///         - `$type`: The type to match.
///         - `$expressions`: The expressions to match against.
/// - `exp_conversion`: Converts the variant to the specified type.
///     - Parameters:
///         - `$type`: The type to convert to.
///         - `$variant`: The variant to convert.
///         - Usage: `exp_conversion!(connector_type, self);`
/// 
/// # Notes:
/// - The `Null` variant is used to represent a null value.
/// - The `ConnectorTypeId` struct contains const identifiers for each variant.
/// - The `ConnectorType` enum contains variants for different types of data.
/// - When creating a new variant, ensure that the `exp_conversion` macro is updated to include the new variant.
/// - Ensure macros are added for the new variant to implement arithmetic, equality, and comparison operations.
/// - Update test cases to include the new variant.

#[derive(Clone, PartialEq, Debug)]
pub enum ConnectorType {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Null(Null)
}

pub struct ConnectorTypeId {}

impl ConnectorTypeId {
    pub const INT: i32 = 1;
    pub const FLOAT: i32 = 2;
    pub const STRING: i32 = 3;
    pub const BOOL: i32 = 4;
    pub const NULL: i32 = 5;
}

impl ConnectorType {
    pub fn new<T>(value: T) -> ConnectorType
    where
        T: Into<ConnectorType>
    {
        value.into()
    }

    pub fn get_type(&self) -> i32 {
        match self {
            ConnectorType::Integer(_) => ConnectorTypeId::INT,
            ConnectorType::Float(_) => ConnectorTypeId::FLOAT,
            ConnectorType::String(_) => ConnectorTypeId::STRING,
            ConnectorType::Boolean(_) => ConnectorTypeId::BOOL,
            ConnectorType::Null(_) => ConnectorTypeId::NULL,
        }
    }

    pub fn get_variant(&self, connector_type: i32) -> ConnectorType {
        exp_conversion!(connector_type, self)
    }
}

impl Eq for ConnectorType {}

impl PartialOrd for ConnectorType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ConnectorType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ConnectorType::Integer(lhs), ConnectorType::Integer(rhs)) => lhs.cmp(rhs),
            (ConnectorType::Float(lhs), ConnectorType::Float(rhs)) => lhs.partial_cmp(rhs).unwrap_or(Ordering::Equal),
            (ConnectorType::String(lhs), ConnectorType::String(rhs)) => lhs.cmp(rhs),
            (ConnectorType::Boolean(lhs), ConnectorType::Boolean(rhs)) => lhs.cmp(rhs),
            (ConnectorType::Null(_), ConnectorType::Null(_)) => Ordering::Equal,
            _ => Ordering::Equal,
        }
    }
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectorType::Integer(value) => write!(f, "{}", value),
            ConnectorType::Float(value) => write!(f, "{}", value),
            ConnectorType::String(value) => write!(f, "{}", value),
            ConnectorType::Boolean(value) => write!(f, "{}", value),
            ConnectorType::Null(_) => write!(f, "Null"),
        }
    }
}

// Macro implementation
impl_arithmetic!(Add, add, +);
impl_arithmetic!(Sub, sub, -);
impl_arithmetic!(Mul, mul, *);
impl_arithmetic!(Div, div, /);
impl_arithmetic!(Rem, rem, %);

impl_to_connectortype!(i32, ConnectorType::Integer);
impl_to_connectortype!(f32, ConnectorType::Float);
impl_to_connectortype!(String, ConnectorType::String);
impl_to_connectortype!(&str, |value: &str| ConnectorType::String(value.to_string()));
impl_to_connectortype!(bool, ConnectorType::Boolean);

impl_partial_eq!(Integer, i32);
impl_partial_eq!(Float, f32);
impl_partial_eq!(String, String);
impl_partial_eq!(String, &str);
impl_partial_eq!(Boolean, bool);
impl_partial_eq!(Null, Null);

impl_ordering!(Integer, i32);
impl_ordering!(Float, f32);

#[cfg(test)]
mod value_test {
    use super::*;

    #[test]
    fn test_conversion() {
        let int = 5;
        let float = 5.0;
        let string = "Hello";
        let boolean = true;

        let int_type = ConnectorType::new(int);
        let float_type = ConnectorType::new(float);
        let string_type = ConnectorType::new(string);
        let boolean_type = ConnectorType::new(boolean);

        assert_eq!(int_type, ConnectorType::Integer(5));
        assert_eq!(float_type, ConnectorType::Float(5.0));
        assert_eq!(string_type, ConnectorType::String("Hello".to_string()));
        assert_eq!(boolean_type, ConnectorType::Boolean(true));

        assert_eq!(int, 5);
        assert_eq!(float, 5.0);
        assert_eq!(string, "Hello");
        assert_eq!(boolean, true);

        assert_eq!(int_type, 5);
        assert_eq!(float_type, 5.0);
        assert_eq!(string_type, "Hello");
        assert_eq!(boolean_type, true);

        let int_to_float = int_type.get_variant(ConnectorTypeId::FLOAT);
        let int_to_string = int_type.get_variant(ConnectorTypeId::STRING);
        let int_to_boolean = int_type.get_variant(ConnectorTypeId::BOOL);
        let int_to_null = int_type.get_variant(ConnectorTypeId::NULL);

        let float_to_int = float_type.get_variant(ConnectorTypeId::INT);
        let float_to_string = float_type.get_variant(ConnectorTypeId::STRING);
        let float_to_boolean = float_type.get_variant(ConnectorTypeId::BOOL);
        let float_to_null = float_type.get_variant(ConnectorTypeId::NULL);

        let string_to_int = string_type.get_variant(ConnectorTypeId::INT);
        let string_to_float = string_type.get_variant(ConnectorTypeId::FLOAT);
        let string_to_boolean = string_type.get_variant(ConnectorTypeId::BOOL);
        let string_to_null = string_type.get_variant(ConnectorTypeId::NULL);

        let boolean_to_int = boolean_type.get_variant(ConnectorTypeId::INT);
        let boolean_to_float = boolean_type.get_variant(ConnectorTypeId::FLOAT);
        let boolean_to_string = boolean_type.get_variant(ConnectorTypeId::STRING);
        let boolean_to_null = boolean_type.get_variant(ConnectorTypeId::NULL);

        assert_eq!(int_to_float, 5.0);
        assert_eq!(int_to_string, "5");
        assert_eq!(int_to_boolean, true);
        assert_eq!(int_to_null, Null::new());

        assert_eq!(float_to_int, 5);
        assert_eq!(float_to_string, "5");
        assert_eq!(float_to_boolean, true);
        assert_eq!(float_to_null, Null::new());

        assert_eq!(string_to_int, 0);
        assert_eq!(string_to_float, 0.0);
        assert_eq!(string_to_boolean, false);
        assert_eq!(string_to_null, Null::new());

        assert_eq!(boolean_to_int, 1);
        assert_eq!(boolean_to_float, 1.0);
        assert_eq!(boolean_to_string, "true");
        assert_eq!(boolean_to_null, Null::new());

        let null = ConnectorType::Null(Null::new());
        let null_to_int = null.get_variant(ConnectorTypeId::INT);
        let null_to_float = null.get_variant(ConnectorTypeId::FLOAT);
        let null_to_string = null.get_variant(ConnectorTypeId::STRING);
        let null_to_boolean = null.get_variant(ConnectorTypeId::BOOL);

        assert_eq!(null_to_int, ConnectorType::Integer(0));
        assert_eq!(null_to_float, ConnectorType::Float(0.0));
        assert_eq!(null_to_string, ConnectorType::String("".to_string()));
        assert_eq!(null_to_boolean, ConnectorType::Boolean(false));
    }

    #[test]
    fn test_arithmetic() {
        let int1 = ConnectorType::new(5);
        let int2 = ConnectorType::new(10);
        let float1 = ConnectorType::new(5.0);
        let float2 = ConnectorType::new(10.0);

        let int_add = int1.clone() + int2.clone();
        let int_sub = int1.clone() - int2.clone();
        let int_mul = int1.clone() * int2.clone();
        let int_div = int1.clone() / int2.clone();
        let int_rem = int1.clone() % int2.clone();

        let float_add = float1.clone() + float2.clone();
        let float_sub = float1.clone() - float2.clone();
        let float_mul = float1.clone() * float2.clone();
        let float_div = float1.clone() / float2.clone();
        let float_rem = float1.clone() % float2.clone();

        assert_eq!(int_add, 15);
        assert_eq!(int_sub, -5);
        assert_eq!(int_mul, 50);
        assert_eq!(int_div, 0);
        assert_eq!(int_rem, 5);

        assert_eq!(float_add, 15.0);
        assert_eq!(float_sub, -5.0);
        assert_eq!(float_mul, 50.0);
        assert_eq!(float_div, 0.5);
        assert_eq!(float_rem, 5.0);
    }

    #[test]
    fn test_comparison() {
        let int1 = ConnectorType::new(5);
        let int2 = ConnectorType::new(10);
        let float1 = ConnectorType::new(5.0);
        let float2 = ConnectorType::new(10.0);
        let string1 = ConnectorType::new("Hello");
        let boolean1 = ConnectorType::new(true);

        let int_inner = 5;
        let float_inner = 5.0;
        let string_inner = "Hello";
        let boolean_inner = true;

        assert_eq!(int1, int_inner, "Integer equals comparison failed");
        assert_eq!(float1, float_inner, "Float equals comparison failed");
        assert_eq!(string1, string_inner, "String equals comparison failed");
        assert_eq!(boolean1, boolean_inner, "Boolean equals comparison failed");

        assert_eq!(int_inner, int1, "Integer equals comparison failed");
        assert_eq!(float_inner, float1, "Float equals comparison failed");
        assert_eq!(string_inner, string1, "String equals comparison failed");
        assert_eq!(boolean_inner, boolean1, "Boolean equals comparison failed");

        assert!(int1 < int2, "Integer less than comparison failed");
        assert!(float1 < float2, "Float less than comparison failed");
        
        assert!(int2 > int1, "Integer greater than comparison failed");
        assert!(float2 > float1, "Float greater than comparison failed");

        assert!(int1 <= int_inner, "Integer less than or equal comparison failed");
        assert!(float1 <= float_inner, "Float less than or equal comparison failed");

        assert!(int_inner >= int1, "Integer greater than or equal comparison failed");
        assert!(float_inner >= float1, "Float greater than or equal comparison failed");
        
        assert!(int1 != int2, "Integer not equal comparison failed");
        assert!(float1 != float2, "Float not equal comparison failed");

        assert!(int1 == int_inner, "Integer equal comparison failed");
        assert!(float1 == float_inner, "Float equal comparison failed");
    }

    // Fail tests
    #[test]
    #[should_panic]
    fn test_conversion_fail() {
        let int = 5;
        let int_type = ConnectorType::new(int);
        let _ = int_type.get_variant(6);
    }

    #[test]
    #[should_panic]
    fn test_arithmetic_fail() {
        let int = ConnectorType::new(5);
        let float = ConnectorType::new(5.0);
        let _ = int + float;
    }
}