// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Display;

/// Represents a Scala primitive type
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
