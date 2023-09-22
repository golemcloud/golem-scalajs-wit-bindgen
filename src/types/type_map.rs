use std::collections::HashMap;

use id_arena::Id;
use wit_parser::{TypeDef, TypeDefKind, UnresolvedPackage};

use super::{Type, TypeName};

pub struct TypeMap(HashMap<Id<TypeDef>, TypeName>);

impl TypeMap {
    pub fn get(&self, k: &Id<TypeDef>) -> Option<&TypeName> {
        self.0.get(k)
    }
}

impl From<&UnresolvedPackage> for TypeMap {
    fn from(unresolved_package: &UnresolvedPackage) -> Self {
        let hash_map1 = unresolved_package
            .types
            .iter()
            .filter_map(|(id, ty)| ty.name.clone().map(|name| (id, TypeName::from(name))))
            .collect::<HashMap<_, _>>();

        let hash_map2 = unresolved_package
            .types
            .iter()
            .filter_map(|(id, ty)| {
                if ty.name.is_none() {
                    match ty.kind {
                        TypeDefKind::List(ty) => Some((
                            id,
                            TypeName::from(format!(
                                "List[{}]",
                                Type::from_wit(ty, &Self(hash_map1.clone()))
                            )), // TODO fix case
                        )),
                        _ => todo!("Support other kinds of types"),
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        Self(hash_map1.into_iter().chain(hash_map2.into_iter()).collect())
    }
}
