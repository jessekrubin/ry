use pyo3::prelude::*;
pub mod ipaddr;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ipaddr::PyIpv4Addr>()?;
    m.add_class::<ipaddr::PyIpv6Addr>()?;
    m.add_class::<ipaddr::PyIpAddr>()?;
    Ok(())
}
