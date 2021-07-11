use std::ops::Range;
use std::cmp::Ordering;

use rand::{Rng, prelude::ThreadRng};

use crate::{XES, to_xes_format};




pub trait Number: Sized {
    fn number(num: f32) -> Self;
    fn exp(self: Self) -> Self;
    fn pow(self: Self, rhs: Self) -> Self;
    fn mul(self: Self, rhs: Self) -> Self;
    fn add(self: Self, rhs: Self) -> Self;
    fn sub(self: Self, rhs: Self) -> Self;
    fn div(self: Self, rhs: Self) -> Self;
    fn comp(self: Self, rhs: Self) -> Option<Ordering>;
    fn rand(r: Range<Self>, thread: &mut ThreadRng) -> Self;
}

//here I could use a macro - (not actually but ok)

impl Number for XES {
    fn number(num: f32) -> Self {
        to_xes_format(num)
    }

    fn exp(self: Self) -> Self {
        XES::exp(&self)
    }

    fn pow(self: Self, rhs: Self) -> Self {
        XES::pow(&self, rhs.to_f32())
    }

    fn mul(self: Self, rhs: Self) -> Self {
        self*rhs
    }

    fn add(self: Self, rhs: Self) -> Self {
        self+rhs
    }

    fn sub(self: Self, rhs: Self) -> Self {
        self-rhs
    }

    fn div(self: Self, rhs: Self) -> Self {
        self/rhs
    }

    fn comp(self: Self, rhs: Self) -> Option<Ordering> {
        todo!()
    }

    fn rand(r: Range<Self>, rng: &mut ThreadRng) -> Self {
        let num: f32 = rng.gen_range(0f32..1000f32)*2.-1000.;
        if num > 0. {
            XES {number: ((num as i16)*10)+3}
        } else {
            XES {number: ((num as i16)*10)-3}
        }
    }
}

impl Number for i8 {
    fn number(num: f32) -> Self {
        num as i8
    }

    fn exp(self: Self) -> Self {
        (self as f32).exp() as i8
    }

    fn pow(self: Self, rhs: Self) -> Self {
        i8::pow(self, rhs as u32)
    }

    fn mul(self: Self, rhs: Self) -> Self {
        self*rhs
    }

    fn add(self: Self, rhs: Self) -> Self {
        self+rhs
    }

    fn sub(self: Self, rhs: Self) -> Self {
        self-rhs
    }

    fn div(self: Self, rhs: Self) -> Self {
        self-rhs
    }

    fn comp(self: Self, rhs: Self) -> Option<Ordering> {
        todo!()
    }

    fn rand(r: Range<Self>, thread: &mut ThreadRng) -> Self {
        thread.gen_range(-50..50)
    }
}

impl Number for f32 {
    
    fn number(num: f32) -> Self {
        num
    }
    fn exp(self: f32) -> f32 {
        self.exp()
    }
    fn pow(self: f32, rhs: f32) -> f32 {
        self.powf(rhs)
    }
    fn mul(self: f32, rhs: f32) -> f32 {
        self*rhs
    }
    fn div(self: f32, rhs: f32) -> f32 {
        self/rhs
    }
    fn add(self: f32, rhs: f32) -> f32 {
        self+rhs
    }
    fn sub(self: f32, rhs: f32) -> f32 {
        self-rhs
    }
    fn comp(self: f32, rhs: f32) -> Option<Ordering> {
        self.partial_cmp(&rhs)
    }
    fn rand(r: Range<Self>, rng: &mut ThreadRng) -> Self {
        rng.gen_range(r.clone())*2.-1.
    }
}

impl Number for f64 {
    fn number(num: f32) -> Self {
        num as f64
    }
    fn exp(self: f64) -> f64 {
        self.exp()
    }

    fn pow(self: f64, rhs: f64) -> f64 {
        self.powf(rhs)
    }
    fn mul(self: f64, rhs: f64) -> f64 {
        self*rhs
    }
    fn div(self: f64, rhs: f64) -> f64 {
        self/rhs
    }
    fn add(self: f64, rhs: f64) -> f64 {
        self+rhs
    }
    fn sub(self: f64, rhs: f64) -> f64 {
        self-rhs
    }
    fn comp(self: f64, rhs: f64) -> Option<Ordering> {
        self.partial_cmp(&rhs)
    }
    fn rand(r: Range<Self>, rng: &mut ThreadRng) -> Self {
        rng.gen_range(r.clone())*2.-1.
    }
}
