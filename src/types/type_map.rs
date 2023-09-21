use std::collections::HashMap;

use id_arena::Id;
use wit_parser::{Interface, TypeDef};

use super::TypeName;

pub struct TypeMap(HashMap<Id<TypeDef>, TypeName>);

impl TypeMap {
    pub fn get(&self, k: &Id<TypeDef>) -> Option<&TypeName> {
        self.0.get(k)
    }
}

impl From<Interface> for TypeMap {
    fn from(interface: Interface) -> Self {
        Self(
            interface
                .types
                .into_iter()
                .map(|(name, id)| (id, TypeName::from(name)))
                .collect::<HashMap<_, _>>(),
        )
    }
}
