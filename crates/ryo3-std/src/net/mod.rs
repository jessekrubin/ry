pub use ipaddr::{RyIpAddr, RyIpv4Addr, RyIpv6Addr};
mod from_str;
use pyo3::prelude::*;
pub use socketaddr::{RySocketAddr, RySocketAddrV4, RySocketAddrV6};
mod from;
mod ipaddr;
mod ipaddr_props;
#[cfg(feature = "pydantic")]
mod pydantic;
mod socketaddr;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyIpv4Addr>()?;
    m.add_class::<RyIpv6Addr>()?;
    m.add_class::<RyIpAddr>()?;
    m.add_class::<RySocketAddrV4>()?;
    m.add_class::<RySocketAddrV6>()?;
    m.add_class::<RySocketAddr>()?;
    Ok(())
}
