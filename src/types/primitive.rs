use std::fmt::Display;

pub enum Primitive {
    Boolean,
    Byte,
    Short,
    Integer,
    Long,
    Float,
    Double,
    Char,
    String,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Boolean => write!(f, "Boolean"),
            Primitive::Byte => write!(f, "Byte"),
            Primitive::Short => write!(f, "Short"),
            Primitive::Integer => write!(f, "Integer"),
            Primitive::Long => write!(f, "Long"),
            Primitive::Float => write!(f, "Float"),
            Primitive::Double => write!(f, "Double"),
            Primitive::Char => write!(f, "Char"),
            Primitive::String => write!(f, "String"),
        }
    }
}
