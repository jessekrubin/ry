#![allow(dead_code)]
#![allow(unused_variables)]
use pyo3::{pyfunction, PyResult};

pub(crate) fn jiffdev() -> Result<(), String> {
    let t1 = jiff::civil::time(1, 2, 3, 4);
    println!("t1: {t1:?}");
    let t2 = jiff::civil::time(1, 2, 3, 4);
    println!("t2: {t2:?}");
    let sub = t1 - t2;
    println!("sub: {sub:?}");

    // let times_Added = t1 + t2;
    let added_span = t1 + sub;
    println!("added_span: {added_span:?}");
    let time_minus_span = t1 - sub;
    println!("time_minus_span: {time_minus_span:?}");

    let as_duration = sub
        .to_jiff_duration(jiff::civil::date(2021, 1, 1))
        .map_err(|e| format!("Error converting to jiff duration: {e}"))?;
    println!("as_duration: {as_duration:?}");

    let t_minus_duration = t1 - as_duration;
    //

    let dt1 = jiff::civil::datetime(2021, 1, 1, 1, 2, 3, 4);
    println!("dt1: {dt1:?}");
    let dt2 = jiff::civil::datetime(2021, 1, 1, 1, 2, 3, 4);
    println!("dt2: {dt2:?}");
    let sub = dt1 - dt2;
    Ok(())
}
#[pyfunction]
pub(crate) fn pyjiffdev() -> PyResult<()> {
    let res = jiffdev();
    match res {
        Ok(()) => Ok(()),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}
