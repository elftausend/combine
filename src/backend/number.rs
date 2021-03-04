use std::ops::Range;
use std::cmp::Ordering;

use rand::{Rng, prelude::ThreadRng};




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

//here I could use a macro

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
