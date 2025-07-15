use pyo3::FromPyObject;

#[derive(Debug, Clone, PartialEq, FromPyObject)]
pub enum StringOrStrings {
    String(String),
    Strings(Vec<String>),
}

impl From<StringOrStrings> for Vec<String> {
    fn from(sos: StringOrStrings) -> Self {
        match sos {
            StringOrStrings::String(s) => vec![s],
            StringOrStrings::Strings(v) => v,
        }
    }
}
