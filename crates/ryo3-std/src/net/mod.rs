pub use ipaddr::{PyIpAddr, PyIpv4Addr, PyIpv6Addr};
use pyo3::prelude::*;
pub use socketaddr::{PySocketAddr, PySocketAddrV4, PySocketAddrV6};
mod from;
mod ipaddr;
mod ipaddr_props;
mod socketaddr;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIpv4Addr>()?;
    m.add_class::<PyIpv6Addr>()?;
    m.add_class::<PyIpAddr>()?;
    m.add_class::<PySocketAddrV4>()?;
    m.add_class::<PySocketAddrV6>()?;
    m.add_class::<PySocketAddr>()?;
    Ok(())
}
