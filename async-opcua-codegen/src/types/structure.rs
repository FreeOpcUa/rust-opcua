#[derive(serde::Serialize, Debug)]
pub enum StructureFieldType {
    Field(String),
    Array(String),
}

#[derive(serde::Serialize, Debug)]
pub struct StructureField {
    pub name: String,
    pub original_name: String,
    pub typ: StructureFieldType,
}

#[derive(serde::Serialize, Debug)]
pub struct StructuredType {
    pub name: String,
    pub fields: Vec<StructureField>,
    pub hidden_fields: Vec<String>,
    pub documentation: Option<String>,
    pub base_type: Option<String>,
    pub is_union: bool,
}

impl StructuredType {
    pub fn visible_fields(&self) -> impl Iterator<Item = &StructureField> {
        self.fields
            .iter()
            .filter(|f| !self.hidden_fields.contains(&f.name))
    }
}
