
use crate::backend::datatypes::*;



pub trait Math: Clone + Send {
    fn exp(data: Self) -> Self;
    fn sum(data: Self, pre_sum: f32) -> f32;
    fn convert_to_format(data: f32) -> Self;
    fn pow(data: Self, power: f32) -> Self;
}



impl Math for f32 {
    fn exp(data: Self) -> Self {
        data.exp()
    }
    fn sum(data: f32, pre_sum: f32) -> f32 {
        data+pre_sum
    }
    #[inline]
    fn convert_to_format(data: Self) -> Self {
        data
    }
    fn pow(data: Self, power: f32) -> Self {
        data.powf(power)
    }
}

impl Math for f64 {
    fn exp(data: Self) -> Self {
        data.exp()
    }
    fn sum(data: f64, pre_sum: f32) -> f32 {
        data as f32+pre_sum
    }
    #[inline]
    fn convert_to_format(data: f32) -> Self {
        data as f64
    }
    fn pow(data: Self, power: f32) -> Self {
        data.powf(power as f64)
    }
}

impl Math for XES {
    fn exp(data: Self) -> Self {
        data.exp()
    }
    fn sum(data: XES, pre_sum: f32) -> f32 {
        data.to_f32()+pre_sum
    }
    #[inline]
    fn convert_to_format(data: f32) -> Self {
        to_xes_format(data)
    }
    fn pow(data: Self, power: f32) -> Self {
        data.pow(power)
    }
}

impl <T>std::iter::Sum<&'static T> for XES {
    fn sum<I: Iterator<Item = &'static T>>(iter: I) -> Self {
        todo!()
    }
}

impl Math for UES {
    fn exp(data: Self) -> Self {
        data.exp()
    }
    fn sum(data: UES, pre_sum: f32) -> f32 {
        data.to_decimal()+pre_sum
    }
    fn convert_to_format(data: f32) -> Self {
        to_ues_format(data)
    }
    fn pow(data: Self, power: f32) -> Self {
        data.pow(power)
    }
}

impl <T>std::iter::Sum<&'static T> for UES {
    fn sum<I: Iterator<Item = &'static T>>(iter: I) -> Self {
        todo!()
    }
}


