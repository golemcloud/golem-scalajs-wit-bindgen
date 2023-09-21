use std::fmt::Display;

use convert_case::{Case, Casing};

use wit_parser::{Field as WitField, Record as WitRecord};

use crate::types::{Type, TypeMap, TypeName};

pub struct FieldName(String);

impl Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FieldName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

pub struct Field {
    name: FieldName,
    ty: Type,
}

impl Field {
    pub fn from_wit(field: WitField, type_map: &TypeMap) -> Self {
        Self {
            name: FieldName::from(field.name),
            ty: Type::from_wit(field.ty, type_map),
        }
    }
}

pub struct Record {
    name: TypeName,
    fields: Vec<Field>,
}

impl Record {
    pub fn from_wit(name: &str, record: &WitRecord, type_map: &TypeMap) -> Self {
        Self {
            name: TypeName::from(name.to_owned()),
            fields: record
                .clone()
                .fields
                .into_iter()
                .map(|field| Field::from_wit(field, type_map))
                .collect::<Vec<_>>(),
        }
    }

    pub fn render(self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}: {ty}"))
            .collect::<Vec<_>>()
            .join("\n");

        let apply_params = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");

        let apply_temp_vars = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}0: {ty} = {name}"))
            .collect::<Vec<_>>()
            .join("\n");

        let new_vars = self
            .fields
            .iter()
            .map(|Field { name, ty }| format!("val {name}: {ty} = {name}0"))
            .collect::<Vec<_>>()
            .join("\n");

        let name = self.name;

        format!(
            "
                sealed trait {name} extends js.Object {{
                    {fields}
                }}
                object {name} {{
                    def apply({apply_params}): {name} = {{
                        {apply_temp_vars}

                        new {name} {{
                            {new_vars}
                        }}
                    }}
                }}
            "
        )
    }
}
