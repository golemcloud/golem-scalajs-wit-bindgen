use std::collections::HashMap;

use id_arena::Id;
use wit_parser::{Result_, TypeDef, TypeDefKind, UnresolvedPackage};

use super::{ConcreteName, Constructor, TypeName};

/// Contains mappings from type IDs to type names
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
            .filter_map(|(id, ty)| {
                ty.name
                    .clone()
                    .map(|name| (id, TypeName::Concrete(ConcreteName::from(name))))
            })
            .collect::<HashMap<_, _>>();

        let hash_map2 = unresolved_package
            .types
            .iter()
            .filter_map(|(id, ty)| {
                if ty.name.is_none() {
                    match ty.kind {
                        TypeDefKind::List(ty) => Some((
                            id,
                            TypeName::Constructor(
                                Constructor::new("js.Array", vec![ty], &TypeMap(hash_map1.clone()))
                                    .ok()?,
                            ),
                        )),
                        TypeDefKind::Option(ty) => Some((
                            id,
                            TypeName::Constructor(
                                Constructor::new(
                                    "js.UndefOr",
                                    vec![ty],
                                    &TypeMap(hash_map1.clone()),
                                )
                                .ok()?,
                            ),
                        )),
                        TypeDefKind::Result(Result_ {
                            ok: Some(ok),
                            err: Some(err),
                        }) => Some((
                            id,
                            TypeName::Constructor(
                                Constructor::new(
                                    "Result",
                                    vec![ok, err],
                                    &TypeMap(hash_map1.clone()),
                                )
                                .ok()?,
                            ),
                        )),
                        _ => todo!("Support other kinds of constructors"),
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        Self(hash_map1.into_iter().chain(hash_map2).collect())
    }
}
