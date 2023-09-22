use std::fmt::Display;

use convert_case::{Case, Casing};

use wit_parser::{Case as WitCase, Variant as WitVariant};

use crate::types::{Type, TypeMap, TypeName};

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

struct VariantCase {
    name: CaseName,
    ty: Option<Type>,
}

impl VariantCase {
    pub fn from_wit(case: WitCase, type_map: &TypeMap) -> Self {
        Self {
            name: CaseName::from(case.name),
            ty: case.ty.map(|ty| Type::from_wit(ty, type_map)),
        }
    }
}

pub struct Variant {
    name: TypeName,
    cases: Vec<VariantCase>,
}

impl Variant {
    pub fn from_wit(name: &str, variant: &WitVariant, type_map: &TypeMap) -> Self {
        Self {
            name: TypeName::from(name.to_owned()),
            cases: variant
                .cases
                .iter()
                .map(|case| VariantCase::from_wit(case.clone(), type_map))
                .collect(),
        }
    }

    pub fn render(self) -> String {
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

        format!(
            "
                sealed trait {name} extends js.Object {{ self =>
                    val tag: String
                    val `val`: js.UndefOr[js.Object] = js.undefined
                }}

                object {name} {{
                    {constructors}
                }}
            "
        )
    }
}
