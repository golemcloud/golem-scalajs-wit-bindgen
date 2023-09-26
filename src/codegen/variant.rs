use std::fmt::Display;

use color_eyre::Result;
use convert_case::{Case, Casing};
use wit_parser::{Case as WitCase, Variant as WitVariant};

use super::Render;
use crate::types::{ConcreteName, Type, TypeMap, TypeName};

/// Represents the name of a variant case in Scala
struct CaseName(String);

impl Display for CaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CaseName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

/// Represents a variant case in Scala
struct VariantCase {
    /// The case name
    name: CaseName,

    /// The internal case type
    ty: Option<Type>,
}

impl VariantCase {
    /// Constructs a `VariantCase` from WIT
    pub fn from_wit(case: WitCase, type_map: &TypeMap) -> Result<Self> {
        Ok(Self {
            name: CaseName::from(case.name),
            ty: match case.ty {
                Some(ty) => Some(Type::from_wit(ty, type_map)?),
                None => None,
            },
        })
    }
}

/// Represents a variant in Scala
pub struct Variant {
    /// The variant name
    name: TypeName,

    /// The variant cases
    cases: Vec<VariantCase>,
}

impl Variant {
    /// Constructs a `Variant` from WIT
    pub fn from_wit(name: &str, variant: &WitVariant, type_map: &TypeMap) -> Result<Self> {
        let cases: Result<Vec<VariantCase>> = variant
            .cases
            .iter()
            .map(|case| VariantCase::from_wit(case.clone(), type_map))
            .collect();

        Ok(Self {
            name: TypeName::Concrete(ConcreteName::from(name.to_owned())),
            cases: cases?,
        })
    }
}

impl Render for Variant {
    fn render(self) -> Result<String> {
        let name = self.name;

        let constructors = self
            .cases
            .iter()
            .map(
                |VariantCase {
                     name: case_name,
                     ty,
                 }| {
                    let (def_or_val, param_list, val) = if let Some(ty) = ty {
                        (
                            "def",
                            format!("(value: {ty})"),
                            "override val `val`: js.UndefOr[js.Object] = js.Object(value)",
                        )
                    } else {
                        ("val", String::new(), "")
                    };

                    format!(
                        "
                            {def_or_val} {case_name}{param_list}: {name} = new {name} {{
                                val tag: String = \"{case_name}\"
                                {val}
                            }}
                        "
                    )
                },
            )
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!(
            "
                sealed trait {name} extends js.Object {{ self =>
                    val tag: String
                    val `val`: js.UndefOr[js.Object] = js.undefined
                }}

                object {name} {{
                    {constructors}
                }}
            "
        ))
    }
}
