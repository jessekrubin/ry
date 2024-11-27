use heck::{
    self, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase,
    ToSnakeCase, ToSnekCase, ToTitleCase, ToTrainCase,
};
use pyo3::prelude::*;

macro_rules! ryheckfn {
    ($name:ident, $case:ident) => {
        #[pyfunction]
        #[pyo3(text_signature = "(string: str)")]
        fn $name(string: &str) -> String {
            string.$case().to_string()
        }
    };
}

ryheckfn!(kebab_case, to_kebab_case);
ryheckfn!(camel_case, to_lower_camel_case);
ryheckfn!(pascal_case, to_pascal_case);
ryheckfn!(shouty_kebab_case, to_shouty_kebab_case);
ryheckfn!(shouty_snake_case, to_shouty_snake_case);
ryheckfn!(snake_case, to_snake_case);
ryheckfn!(snek_case, to_snek_case);
ryheckfn!(title_case, to_title_case);
ryheckfn!(train_case, to_train_case);

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
