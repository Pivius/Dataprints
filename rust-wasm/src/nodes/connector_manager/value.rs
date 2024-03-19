pub enum ConnectorType {
    Integer,
    Float,
    String,
    Boolean,
}

impl core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConnectorType::Integer => write!(f, "Integer"),
            ConnectorType::Float => write!(f, "Float"),
            ConnectorType::String => write!(f, "String"),
            ConnectorType::Boolean => write!(f, "Boolean"),
        }
    }
}

impl PartialEq for ConnectorType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ConnectorType::Integer => match other {
                ConnectorType::Integer => true,
                _ => false,
            },
            ConnectorType::Float => match other {
                ConnectorType::Float => true,
                _ => false,
            },
            ConnectorType::String => match other {
                ConnectorType::String => true,
                _ => false,
            },
            ConnectorType::Boolean => match other {
                ConnectorType::Boolean => true,
                _ => false,
            },
        }
    }
}

pub trait ConnectorValue {
    fn get_connector_type(&self) -> ConnectorType;
    fn stringify(&self) -> String;
    fn to_i32(&self) -> i32;
    fn to_f32(&self) -> f32;
    fn to_bool(&self) -> bool;
    fn box_self(&self) -> Box<dyn ConnectorValue>;
}

impl ConnectorValue for i32 {
    fn get_connector_type(&self) -> ConnectorType {
        ConnectorType::Integer
    }

    fn stringify(&self) -> String {
        self.to_string()
    }

    fn to_i32(&self) -> i32 {
        *self
    }

    fn to_f32(&self) -> f32 {
        *self as f32
    }

    fn to_bool(&self) -> bool {
        *self != 0
    }

    fn box_self(&self) -> Box<dyn ConnectorValue>{
        Box::new(*self)
    }
}

impl ConnectorValue for f32 {
    fn get_connector_type(&self) -> ConnectorType {
        ConnectorType::Float
    }

    fn stringify(&self) -> String {
        self.to_string()
    }

    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_f32(&self) -> f32 {
        *self
    }

    fn to_bool(&self) -> bool {
        *self != 0.0
    }

    fn box_self(&self) -> Box<dyn ConnectorValue>{
        Box::new(*self)
    }
}

impl ConnectorValue for String {
    fn get_connector_type(&self) -> ConnectorType {
        ConnectorType::String
    }

    fn stringify(&self) -> String {
        self.to_string()
    }

    fn to_i32(&self) -> i32 {
        self.parse().unwrap()
    }

    fn to_f32(&self) -> f32 {
        self.parse().unwrap()
    }

    fn to_bool(&self) -> bool {
        self.parse().unwrap()
    }

    fn box_self(&self) -> Box<dyn ConnectorValue>{
        Box::new(self.to_string())
    }
}

impl ConnectorValue for bool {
    fn get_connector_type(&self) -> ConnectorType {
        ConnectorType::Boolean
    }

    fn stringify(&self) -> String {
        self.to_string()
    }

    fn to_i32(&self) -> i32 {
        if *self {
            1
        } else {
            0
        }
    }

    fn to_f32(&self) -> f32 {
        if *self {
            1.0
        } else {
            0.0
        }
    }

    fn to_bool(&self) -> bool {
        *self
    }

    fn box_self(&self) -> Box<dyn ConnectorValue>{
        Box::new(*self)
    }
}

impl Clone for Box<dyn ConnectorValue> {
    fn clone(&self) -> Box<dyn ConnectorValue> {
        match self.get_connector_type() {
            ConnectorType::Integer => Box::new(self.to_i32()),
            ConnectorType::Float => Box::new(self.to_f32()),
            ConnectorType::String => Box::new(self.stringify()),
            ConnectorType::Boolean => Box::new(self.to_bool()),
        }
    }
}

impl core::fmt::Debug for dyn ConnectorValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

impl PartialEq for dyn ConnectorValue {
    fn eq(&self, other: &Self) -> bool {
        let self_type = self.get_connector_type();
        let other_type = other.get_connector_type();

        if self_type != other_type {
            return false;
        }

        match self_type {
            ConnectorType::Integer => self.to_i32() == other.to_i32(),
            ConnectorType::Float => self.to_f32() == other.to_f32(),
            ConnectorType::String => self.stringify() == other.stringify(),
            ConnectorType::Boolean => self.to_bool() == other.to_bool(),
        }
    }
}

impl PartialEq<i32> for dyn ConnectorValue {
    fn eq(&self, other: &i32) -> bool {
        self.get_connector_type() == ConnectorType::Integer && self.to_i32() == *other
    }
}

impl PartialEq<f32> for dyn ConnectorValue {
    fn eq(&self, other: &f32) -> bool {
        self.get_connector_type() == ConnectorType::Float && self.to_f32() == *other
    }
}

impl PartialEq<String> for dyn ConnectorValue {
    fn eq(&self, other: &String) -> bool {
        self.get_connector_type() == ConnectorType::String && self.stringify() == *other
    }
}

impl PartialEq<bool> for dyn ConnectorValue {
    fn eq(&self, other: &bool) -> bool {
        self.get_connector_type() == ConnectorType::Boolean && self.to_bool() == *other
    }
}

impl PartialEq<i32> for Box<dyn ConnectorValue> {
    fn eq(&self, other: &i32) -> bool {
        self.get_connector_type() == ConnectorType::Integer && self.to_i32() == *other
    }
}

impl PartialEq<f32> for Box<dyn ConnectorValue> {
    fn eq(&self, other: &f32) -> bool {
        self.get_connector_type() == ConnectorType::Float && self.to_f32() == *other
    }
}

impl PartialEq<String> for Box<dyn ConnectorValue> {
    fn eq(&self, other: &String) -> bool {
        self.get_connector_type() == ConnectorType::String && self.stringify() == *other
    }
}

impl PartialEq<bool> for Box<dyn ConnectorValue> {
    fn eq(&self, other: &bool) -> bool {
        self.get_connector_type() == ConnectorType::Boolean && self.to_bool() == *other
    }
}