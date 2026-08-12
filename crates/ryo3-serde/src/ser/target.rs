use core::fmt::Debug;

pub trait PySerializeTarget: Copy + Clone + Debug + Default + 'static {
    const KIND: &'static str;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SerdeTarget;

impl PySerializeTarget for SerdeTarget {
    const KIND: &'static str = "serde";
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JsonTarget;

impl PySerializeTarget for JsonTarget {
    const KIND: &'static str = "json";
}
