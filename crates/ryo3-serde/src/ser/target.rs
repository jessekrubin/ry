use core::fmt::Debug;

pub trait PySerializeTarget: Copy + Clone + Debug + Default + 'static {
    const KIND: &'static str;
    const SORT_KEYS: bool = false;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SerdeTarget;

impl PySerializeTarget for SerdeTarget {
    const KIND: &'static str = "serde";
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JsonTarget<const SORT_KEYS: bool = false>;

impl<const SORT_KEYS: bool> PySerializeTarget for JsonTarget<SORT_KEYS> {
    const KIND: &'static str = "json";
    const SORT_KEYS: bool = SORT_KEYS;
}
