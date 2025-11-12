#[cfg(feature = "jiff")]
use jiff::fmt::friendly::Designator;
use pyo3::basic::CompareOp;
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyInt, PyTuple};
use pyo3::{Bound, IntoPyObjectExt, PyAny, PyResult, Python, pyclass, pymethods};
use ryo3_macro_rules::{
    py_key_err, py_overflow_err, py_overflow_error, py_type_err, py_value_err, py_zero_division_err,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Div, Mul};
use std::time::Duration;

const NANOS_PER_SEC: u32 = 1_000_000_000;
const SECS_PER_MINUTE: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;
const MAX_DAYS: u64 = u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY);
const SECS_PER_WEEK: u64 = SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY * DAYS_PER_WEEK;

// jiff
#[cfg(feature = "jiff")]
const TEMPORAL_SPAN_PARSER: jiff::fmt::temporal::SpanParser =
    jiff::fmt::temporal::SpanParser::new();
#[cfg(feature = "jiff")]
const TEMPORAL_SPAN_PRINTER: jiff::fmt::temporal::SpanPrinter =
    jiff::fmt::temporal::SpanPrinter::new();
#[cfg(feature = "jiff")]
const FRIENDLY_SPAN_PARSER: jiff::fmt::friendly::SpanParser =
    jiff::fmt::friendly::SpanParser::new();

// Maybe use?`HumanTime` designator for friendly parser/printer to avoid ambiguous `µ`:w
// REF: https://github.com/jessekrubin/ry/discussions/229#discussioncomment-14928815
// ```txt
// RUF001 String contains ambiguous `µ` (MICRO SIGN). Did you mean `μ` (GREEK SMALL LETTER MU)?
//   --> tests\std\test_duration_str.py:45:59
//    |
// 43 |         max_dur = ry.Duration.MAX
// 44 |         iso_str = max_dur.friendly()
// 45 |         assert iso_str == "5124095576030431h 15s 999ms 999µs 999ns"
//    |                                                           ^
// 46 |         parsed_max_dur = ry.Duration.from_str(iso_str)
// 47 |         assert parsed_max_dur == max_dur
//    |
//
// Found 1 error.
// ```
#[cfg(feature = "jiff")]
const FRIENDLY_SPAN_PRINTER: jiff::fmt::friendly::SpanPrinter =
    jiff::fmt::friendly::SpanPrinter::new();
// const FRIENDLY_SPAN_PRINTER: jiff::fmt::friendly::SpanPrinter = jiff::fmt::friendly::SpanPrinter::new().designator(
// jiff::fmt::friendly::Designator::HumanTime,
// );

#[derive(Copy, Clone, PartialEq)]
#[pyclass(name = "Duration", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyDuration(pub Duration);

impl From<Duration> for PyDuration {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}

impl From<PyDuration> for Option<Duration> {
    fn from(d: PyDuration) -> Self {
        Some(d.0)
    }
}

impl PyDuration {
    fn new(secs: u64, nanos: u32) -> PyResult<Self> {
        if nanos < NANOS_PER_SEC {
            Ok(Self(Duration::new(secs, nanos)))
        } else {
            let secs = secs
                .checked_add(u64::from(nanos / NANOS_PER_SEC))
                .ok_or_else(|| {
                    py_overflow_error!("overflow; seconds part of Duration::new too large")
                })?;
            let nanos = nanos % NANOS_PER_SEC;
            Ok(Self(Duration::new(secs, nanos)))
        }
    }

    #[must_use]
    pub fn inner(&self) -> &Duration {
        &self.0
    }

    fn try_from_secs_f32(secs: f32) -> PyResult<Self> {
        if secs.is_nan() {
            py_value_err!("invalid value: nan")
        } else if secs.is_infinite() {
            if secs.is_sign_negative() {
                py_type_err!("negative duration")
            } else {
                py_overflow_err!("invalid value: inf")
            }
        } else if secs < 0.0 {
            py_type_err!("negative duration")
        } else {
            Duration::try_from_secs_f32(secs)
                .map(Self::from)
                .map_err(|e| py_overflow_error!("{e}"))
        }
    }

    fn try_from_secs_f64(secs: f64) -> PyResult<Self> {
        if secs.is_nan() {
            py_value_err!("invalid value: nan")
        } else if secs.is_infinite() {
            if secs.is_sign_negative() {
                py_type_err!("negative duration")
            } else {
                py_overflow_err!("invalid value: inf")
            }
        } else if secs < 0.0 {
            py_type_err!("negative duration")
        } else {
            Duration::try_from_secs_f64(secs)
                .map(Self::from)
                .map_err(|e| py_overflow_error!("{e}"))
        }
    }
}

#[pymethods]
#[expect(clippy::needless_pass_by_value)]
impl PyDuration {
    #[new]
    #[pyo3(signature = (secs = 0, nanos = 0))]
    fn py_new(secs: u64, nanos: u32) -> PyResult<Self> {
        Self::new(secs, nanos)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.0.as_secs().into_bound_py_any(py)?,
                self.0.subsec_nanos().into_bound_py_any(py)?,
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(Duration::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Duration::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Duration::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SECOND() -> Self {
        Self(Duration::from_secs(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MILLISECOND() -> Self {
        Self(Duration::from_millis(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MICROSECOND() -> Self {
        Self(Duration::from_micros(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NANOSECOND() -> Self {
        Self(Duration::from_nanos(1))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_zero()
    }

    fn __float__(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn __int__(&self) -> u128 {
        self.0.as_nanos()
    }

    // ========================================================================
    // MATHS/OPERATORS
    // ========================================================================
    // OLD VERSION ALLOWING FOR DURATION/DELTA COMPARISONS
    // fn __richcmp__(&self, other: PyDurationComparable, op: CompareOp) -> bool {
    //     match other {
    //         PyDurationComparable::PyDuration(other) => match op {
    //             CompareOp::Eq => self.0 == other.0,
    //             CompareOp::Ne => self.0 != other.0,
    //             CompareOp::Lt => self.0 < other.0,
    //             CompareOp::Le => self.0 <= other.0,
    //             CompareOp::Gt => self.0 > other.0,
    //             CompareOp::Ge => self.0 >= other.0,
    //         },
    //         PyDurationComparable::Duration(other) => match op {
    //             CompareOp::Eq => self.0 == other,
    //             CompareOp::Ne => self.0 != other,
    //             CompareOp::Lt => self.0 < other,
    //             CompareOp::Le => self.0 <= other,
    //             CompareOp::Gt => self.0 > other,
    //             CompareOp::Ge => self.0 >= other,
    //         },
    //     }
    // }
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            self.0
                .checked_add(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!("overflow in Duration addition"))
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: Duration = d.extract()?;
            self.0
                .checked_add(rs_dur)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!("overflow in Duration addition"))
        } else {
            py_type_err!("unsupported operand type(s); must be Duration | datetime.timedelta")
        }
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            self.0
                .checked_sub(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!("overflow in Duration subtraction"))
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: Duration = d.extract()?;
            self.0
                .checked_sub(rs_dur)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!("overflow in Duration subtraction"))
        } else {
            py_type_err!("unsupported operand type(s); must be Duration | datetime.timedelta")
        }
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__sub__(other)
    }

    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(f) = other.cast::<PyInt>() {
            let i = f.extract::<u32>()?;
            if i == 0 {
                return py_zero_division_err!("division by zero");
            }
            self.checked_div(i)
                .ok_or_else(|| py_overflow_error!("overflow in Duration division"))?
                .into_bound_py_any(py)
        } else if let Ok(f) = other.cast::<pyo3::types::PyFloat>() {
            let f = f.extract::<f64>()?;
            self.div_f64(f).and_then(|d| d.into_bound_py_any(py))
        } else if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            self.div_duration_f64(rs_dur)?.into_bound_py_any(py)
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: Duration = d.extract()?;
            if rs_dur.is_zero() {
                py_zero_division_err!()
            } else {
                self.0.div_duration_f64(rs_dur).into_bound_py_any(py)
            }
        } else {
            py_type_err!(
                "unsupported operand type(s); must be int | float | Duration | datetime.timedelta"
            )
        }
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = other.extract::<u32>() {
            self.0
                .checked_mul(i)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!("overflow in Duration multiplication"))
        } else if let Ok(f) = other.extract::<f64>() {
            self.mul_f64(f)
        } else {
            py_overflow_err!("unsupported operand type(s); must be int | float")
        }
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__mul__(other)
    }

    // ========================================================================
    // PROPERTIES/GETTERS
    // ========================================================================
    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[getter]
    fn secs(&self) -> u64 {
        self.0.as_secs()
    }

    #[getter]
    fn nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    #[getter]
    fn nanoseconds(&self) -> u32 {
        self.0.subsec_nanos()
    }

    #[getter]
    fn ns(&self) -> u32 {
        self.0.subsec_nanos()
    }

    #[getter]
    fn days(&self) -> u64 {
        self.0.as_secs() / 86400
    }

    /// Return the number of seconds in the duration counting days
    #[getter]
    fn seconds(&self) -> u64 {
        self.0.as_secs()
    }

    /// Return the seconds % days (self.seconds % 86400)
    #[getter]
    fn seconds_remainder(&self) -> u64 {
        self.0.as_secs() % 86400
    }

    #[getter]
    fn microseconds(&self) -> u32 {
        self.0.subsec_micros()
    }

    #[getter]
    fn subsec_millis(&self) -> u32 {
        self.0.subsec_millis()
    }

    #[getter]
    fn subsec_micros(&self) -> u32 {
        self.0.subsec_micros()
    }

    #[getter]
    fn subsec_nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    // ========================================================================
    // TO-CONVERSIONS
    // ========================================================================
    fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }

    fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    // ========================================================================
    // PYTHON CONVERSIONS
    // ========================================================================

    /// Convert to python `datetime.timedelta`
    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Duration {
        self.0
    }

    /// Convert to python `datetime.timedelta`
    #[expect(clippy::wrong_self_convention)]
    fn to_pytimedelta(&self) -> Duration {
        self.0
    }

    /// Convert from python `datetime.timedelta`
    #[staticmethod]
    fn from_pytimedelta(delta: Duration) -> Self {
        Self(delta)
    }

    // ========================================================================
    // TO/FROM NUMBERS
    // ========================================================================
    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(interns::secs(py), self.0.as_secs())?;
        dict.set_item(interns::nanos(py), self.0.subsec_nanos())?;
        Ok(dict)
    }

    #[staticmethod]
    fn from_dict(dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let secs = dict.get_item(interns::secs(dict.py()))?;
        let nanos = dict.get_item(interns::nanos(dict.py()))?;
        match (secs, nanos) {
            (Some(secs), Some(nanos)) => {
                let secs = secs.extract::<u64>()?;
                let nanos = nanos.extract::<u32>()?;
                Self::new(secs, nanos)
            }
            // (Some(secs), None) => {
            //     let secs = secs.extract::<u64>()?;
            //     Self::new(secs, 0)
            // }
            // (None, Some(nanos)) => {
            //     let nanos = nanos.extract::<u32>()?;
            //     Self::new(0, nanos)
            // }
            _ => py_key_err!("dict must contain 'secs' and/or 'nanos' keys"),
        }
    }

    // ========================================================================
    // FROM NUMBERS
    // ========================================================================

    /// Create a new `Duration` from the specified number of seconds.
    #[staticmethod]
    fn from_secs(secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }

    #[staticmethod]
    fn from_millis(millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }

    #[staticmethod]
    fn from_micros(micros: u64) -> Self {
        Self(Duration::from_micros(micros))
    }

    #[staticmethod]
    fn from_nanos(nanos: u64) -> Self {
        Self(Duration::from_nanos(nanos))
    }

    #[staticmethod]
    fn from_hours(hours: u64) -> PyResult<Self> {
        if hours > u64::MAX / (60 * 60) {
            py_overflow_err!("overflow in Duration::from_hours")
        } else {
            Ok(Self(Duration::from_secs(hours * 60 * 60)))
        }
    }

    #[staticmethod]
    fn from_mins(mins: u64) -> PyResult<Self> {
        if mins > u64::MAX / 60 {
            py_overflow_err!("overflow in Duration::from_mins")
        } else {
            Ok(Self(Duration::from_secs(mins * 60)))
        }
    }

    #[staticmethod]
    fn from_days(days: u64) -> PyResult<Self> {
        if days > MAX_DAYS {
            py_overflow_err!("overflow in Duration::from_days: {days} > {MAX_DAYS}")
        } else {
            Ok(Self(Duration::from_secs(days * 60 * 60 * 24)))
        }
    }

    #[staticmethod]
    fn from_weeks(weeks: u64) -> PyResult<Self> {
        weeks
            .checked_mul(SECS_PER_WEEK)
            .map(|v| Duration::from_secs(v).into())
            .ok_or_else(|| py_overflow_error!("overflow in Duration::from_weeks"))
        // Ok(Self(Duration::from_secs(total)))
    }

    #[staticmethod]
    fn from_secs_f32(secs: f32) -> PyResult<Self> {
        Self::try_from_secs_f32(secs)
    }

    #[staticmethod]
    fn from_secs_f64(secs: f64) -> PyResult<Self> {
        Self::try_from_secs_f64(secs)
    }

    #[cfg(feature = "jiff")]
    #[staticmethod]
    fn fromisoformat(isoformat: &str) -> PyResult<Self> {
        use jiff::fmt::temporal::SpanParser;
        use ryo3_macro_rules::py_value_error;
        let parser = SpanParser::new();
        let duration = parser
            .parse_unsigned_duration(isoformat)
            .map_err(|e| py_value_error!("invalid isoformat string: {e}"))?;
        Ok(Self(duration))
    }

    #[cfg(feature = "jiff")]
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        use ryo3_macro_rules::py_value_error;
        let dur = TEMPORAL_SPAN_PARSER
            .parse_unsigned_duration(s)
            .map(Self::from);
        match dur {
            Ok(dur) => Ok(dur),
            Err(_) => FRIENDLY_SPAN_PARSER
                .parse_unsigned_duration(s)
                .map(Self::from)
                .map_err(|_| py_value_error!("invalid duration string: {s}")),
        }
    }

    // ========================================================================
    // METHODS
    // ========================================================================
    #[cfg(feature = "jiff")]
    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> String {
        self.__str__()
    }

    #[cfg(feature = "jiff")]
    fn __str__(&self) -> String {
        self.isoformat()
    }

    #[cfg(feature = "jiff")]
    fn __format__(&self, fmt: &str) -> PyResult<String> {
        if fmt == "#" {
            Ok(FRIENDLY_SPAN_PRINTER.unsigned_duration_to_string(&self.0))
        } else if fmt.is_empty() {
            Ok(self.py_to_string())
        } else {
            py_type_err!("Invalid format specifier '{fmt}' for SignedDuration")
        }
    }

    #[cfg(feature = "jiff")]
    fn isoformat(&self) -> String {
        TEMPORAL_SPAN_PRINTER.unsigned_duration_to_string(&self.0)
    }

    #[cfg(feature = "jiff")]
    #[pyo3(signature = (designator = None))]
    fn friendly(&self, designator: Option<&str>) -> PyResult<String> {
        if let Some(designator) = designator {
            match designator {
                "human-time" | "human" => Ok(FRIENDLY_SPAN_PRINTER
                    .designator(Designator::HumanTime)
                    .unsigned_duration_to_string(&self.0)),
                "short" => Ok(FRIENDLY_SPAN_PRINTER
                    .designator(Designator::Short)
                    .unsigned_duration_to_string(&self.0)),
                "compact" => Ok(FRIENDLY_SPAN_PRINTER
                    .designator(Designator::Compact)
                    .unsigned_duration_to_string(&self.0)),
                "verbose" => Ok(FRIENDLY_SPAN_PRINTER
                    .designator(Designator::Verbose)
                    .unsigned_duration_to_string(&self.0)),
                other => {
                    py_value_err!(
                        "invalid designator: {other} (expected 'human'/'human-time', 'short', or 'compact')"
                    )
                }
            }
        } else {
            Ok(FRIENDLY_SPAN_PRINTER.unsigned_duration_to_string(&self.0))
        }
    }

    #[pyo3(signature = (interval = None))]
    /// Sleep for the duration
    pub(crate) fn sleep(&self, py: Python<'_>, interval: Option<u64>) -> PyResult<()> {
        let interval = match interval {
            Some(interval) => {
                if interval > 1000 {
                    return py_value_err!("interval must be less than or equal to 1000");
                } else if interval == 0 {
                    return py_value_err!("interval must be greater than 0");
                }
                interval
            }
            None => 10,
        };
        let sleep_duration = self.0;
        let check_interval = Duration::from_millis(interval);
        let mut remaining = sleep_duration;
        while remaining > check_interval {
            py.check_signals()?; // This ensures signals are handled
            py.detach(|| std::thread::sleep(check_interval));
            remaining -= check_interval;
        }
        if remaining > Duration::ZERO {
            py.check_signals()?; // One last signal check before sleeping
            py.detach(|| std::thread::sleep(remaining));
        }

        Ok(())
    }

    // ========================================================================
    // SLEEP CROSSBEAM IMPL
    // ========================================================================
    // pub fn sleep<'py>(&self, py: Python<'py>) -> PyResult<()> {
    //     // channel for sleepytime to signal done
    //     let (tx, rx) = crossbeam_channel::bounded::<()>(1);
    //     let duration = self.0;
    //     std::thread::spawn(move || {
    //         std::thread::sleep(duration);
    //         let _ = tx.send(());
    //     });
    //     loop {
    //         py.check_signals()?;
    //         crossbeam_channel::select! {
    //             recv(rx) -> _ => {
    //                 break;
    //             }
    //             default(Duration::from_millis(10)) => {
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    fn abs_diff(&self, other: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            Ok(Self(self.0.abs_diff(rs_dur.0)))
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            if let Ok(dur) = d.extract::<Duration>() {
                Ok(Self(self.0.abs_diff(dur)))
            } else {
                py_value_err!("cannot compare with negative timedelta")
            }
        } else {
            py_type_err!("unsupported operand type(s); must be Duration | datetime.timedelta")
        }
    }

    fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self::from)
    }

    fn checked_div(&self, other: u32) -> Option<Self> {
        self.0.checked_div(other).map(Self::from)
    }

    fn checked_mul(&self, other: u32) -> Option<Self> {
        self.0.checked_mul(other).map(Self::from)
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from)
    }

    fn div_duration_f32(&self, other: &Self) -> PyResult<f32> {
        if other.0.is_zero() {
            py_zero_division_err!()
        } else {
            Ok(self.0.div_duration_f32(other.0))
        }
    }

    fn div_duration_f64(&self, other: &Self) -> PyResult<f64> {
        if other.0.is_zero() {
            py_zero_division_err!()
        } else {
            Ok(self.0.div_duration_f64(other.0))
        }
    }

    fn div_f32(&self, n: f32) -> PyResult<Self> {
        if n.abs() == 0.0 {
            py_zero_division_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else if n.is_sign_negative() {
            py_type_err!("negative divisor")
        } else {
            let result = self.0.as_secs_f32().div(n);
            Self::try_from_secs_f32(result)
        }
    }

    fn div_f64(&self, n: f64) -> PyResult<Self> {
        if n.abs() == 0.0 {
            py_zero_division_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else if n.is_sign_negative() {
            py_type_err!("negative divisor")
        } else {
            let result = self.0.as_secs_f64().div(n);
            Self::try_from_secs_f64(result)
        }
    }

    fn mul_f32(&self, n: f32) -> PyResult<Self> {
        if n.abs() == 0.0 {
            Ok(Self::from(Duration::ZERO))
        } else if n.is_infinite() {
            py_overflow_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_sign_negative() {
            py_type_err!("negative factor")
        } else {
            let result = self.0.as_secs_f32().mul(n);
            Self::try_from_secs_f32(result)
        }
    }

    fn mul_f64(&self, n: f64) -> PyResult<Self> {
        if n.abs() == 0.0 {
            Ok(Self::from(Duration::ZERO))
        } else if n.is_infinite() {
            py_overflow_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_sign_negative() {
            py_type_err!("negative factor")
        } else {
            let result = self.0.as_secs_f64().mul(n);
            Self::try_from_secs_f64(result)
        }
    }

    fn saturating_add(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_add(other.0))
    }

    fn saturating_mul(&self, other: u32) -> Self {
        Self::from(self.0.saturating_mul(other))
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_sub(other.0))
    }
}

impl std::fmt::Debug for PyDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
    }
}

// ----------------------------------------------------------------------------
// py-string interns
// ----------------------------------------------------------------------------

mod interns {
    use pyo3::prelude::*;
    use ryo3_macro_rules::py_intern_fn;

    py_intern_fn!(secs);
    py_intern_fn!(nanos);
}
