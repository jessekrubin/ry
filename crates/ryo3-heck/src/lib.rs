#![doc = include_str!("../README.md")]
use heck::{
    self, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase,
    ToSnakeCase, ToSnekCase, ToTitleCase, ToTrainCase,
};
use pyo3::prelude::*;

macro_rules! ry_heck_pyfunction {
    ($name:ident, $case:ident) => {
        #[pyfunction]
        #[pyo3(text_signature = "(string: str)")]
        fn $name(string: &str) -> String {
            string.$case().to_string()
        }
    };
}

ry_heck_pyfunction!(kebab_case, to_kebab_case);
ry_heck_pyfunction!(camel_case, to_lower_camel_case);
ry_heck_pyfunction!(pascal_case, to_pascal_case);
ry_heck_pyfunction!(shouty_kebab_case, to_shouty_kebab_case);
ry_heck_pyfunction!(shouty_snake_case, to_shouty_snake_case);
ry_heck_pyfunction!(snake_case, to_snake_case);
ry_heck_pyfunction!(snek_case, to_snek_case);
ry_heck_pyfunction!(title_case, to_title_case);
ry_heck_pyfunction!(train_case, to_train_case);

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kebab_case, m)?)?;
    m.add_function(wrap_pyfunction!(camel_case, m)?)?;
    m.add_function(wrap_pyfunction!(pascal_case, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_kebab_case, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_snake_case, m)?)?;
    m.add_function(wrap_pyfunction!(snake_case, m)?)?;
    m.add_function(wrap_pyfunction!(snek_case, m)?)?;
    m.add_function(wrap_pyfunction!(title_case, m)?)?;
    m.add_function(wrap_pyfunction!(train_case, m)?)?;

    Ok(())
}
