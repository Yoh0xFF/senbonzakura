use std::fmt;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "PascalCase")]
pub enum Type {
    Primitive(PrimitiveType),

    Array(Box<Type>),

    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    Class {
        name: String,
        super_class: Option<String>,
    },

    Generic {
        base: String,
        type_args: Vec<Type>,
    },

    Void,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PrimitiveType {
    Number,
    Boolean,
    String,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Number => write!(f, "Number"),
            PrimitiveType::Boolean => write!(f, "Boolean"),
            PrimitiveType::String => write!(f, "String"),
        }
    }
}
