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
                                Constructor::new(
                                    "WitList",
                                    vec![Some(ty)],
                                    &TypeMap(hash_map1.clone()),
                                )
                                .ok()?,
                            ),
                        )),
                        TypeDefKind::Option(ty) => Some((
                            id,
                            TypeName::Constructor(
                                Constructor::new(
                                    "WitOption",
                                    vec![Some(ty)],
                                    &TypeMap(hash_map1.clone()),
                                )
                                .ok()?,
                            ),
                        )),
                        TypeDefKind::Result(Result_ { ok, err }) => Some((
                            id,
                            TypeName::Constructor(
                                Constructor::new(
                                    "WitResult",
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
