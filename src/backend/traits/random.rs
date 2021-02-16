use rand::{thread_rng, Rng};
use crate::backend::datatypes::*;

pub trait Random: Clone + Send {
    fn create_random() -> Self;
}

impl Random for f32 {
    fn create_random() -> Self {
        let mut rng = thread_rng(); 
        rng.gen_range(0f32..1f32)*2.-1.
    }
}
impl Random for f64 {
    fn create_random() -> Self {
        let mut rng = thread_rng(); 
        rng.gen_range(0f64..1f64)*2.-1.
    }
}

impl Random for XES {
    fn create_random() -> Self {
        let mut rng = thread_rng(); 
        let num: f32 = rng.gen_range(0f32..1000f32)*2.-1000.;
        if num > 0. {
            XES {number: ((num as i16)*10)+3}
        } else {
            XES {number: ((num as i16)*10)-3}
        }
        
    }
}

impl Random for UES {
    fn create_random() -> Self {
        let mut rng = thread_rng();
        let num:f32 = rng.gen_range(10f32..100f32)*2.-100.;
  //      println!("random num: {:?}", num);
        let num = ((num).round()) as i8;
            
        UES {number: num, power: -2}
    }
}