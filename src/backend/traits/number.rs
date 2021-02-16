use crate::{UES, XES, to_ues_format, to_xes_format};





pub trait Number {
    fn number(num: f32) -> Self;
}

impl Number for f32 {
    fn number(num: f32) -> Self {
        num
    }
}

impl Number for f64 {
    fn number(num: f32) -> Self {
        num as f64
    }
}

impl Number for UES {
    fn number(num: f32) -> Self {
        to_ues_format(num)
    }
}
impl Number for XES {
    fn number(num: f32) -> Self {
        to_xes_format(num)
    }
}
