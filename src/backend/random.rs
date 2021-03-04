use std::ops::Range;

use rand::thread_rng;

use super::Matrix;
use super::Number;




pub fn random_matrix<T: 'static>(rows: usize, cols: usize) -> Matrix<T> where T: Number + Send + Clone {
    let mut vec:Vec<T> = Vec::new();
    let len = rows*cols;
    let rng1 = &mut thread_rng(); 
    let range = T::number(0.)..T::number(1.);
    if len <= 560 { //560
        for _x in 0..len {
            vec.push(T::rand(range.clone(), rng1));
        }
        return Matrix {rows, cols, data: vec};
       // return Matrix {rows: self.rows, cols: self.cols, data: vec};  
    }

    let t = (th_r(range.clone(), (len as f32/2.).ceil() as usize, ), th_r(range,(len as f32/2.).floor() as usize,));
    let mut x = t.0.join().unwrap();
    let mut x1 = t.1.join().unwrap();
    vec.append(&mut x);
    vec.append(&mut x1);
    if vec.len() != len {
        panic!("Error in randomizing.. Vector len: {0}, needed len: {1}", vec.len(), len);
    }
    Matrix {rows, cols, data: vec}
//73978
}

pub fn th_r<T: 'static>(range: Range<T>, elements: usize) -> std::thread::JoinHandle<Vec<T>> where T: Number + Send + Clone {
    let builder = std::thread::Builder::new();
    builder.spawn(move || {
        let mut vec: Vec<T> = Vec::new();
        let rng = &mut thread_rng();
        for _x in 0..elements {
            vec.push(T::rand(range.clone(), rng));
        }
        vec 
    }).unwrap()
}
