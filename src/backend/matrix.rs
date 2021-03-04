
use super::number::Number;
use super::random::random_matrix;
use super::sum::th_s;



#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}


impl <T: 'static>Matrix<T> where T: Clone + Copy + Number + Send + std::fmt::Debug {
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        Matrix {rows, cols, data}
    }
    pub fn new(rows: usize, cols: usize, value: T) -> Matrix<T> {
        Matrix {rows, cols, data: vec!(value; rows*cols)}
    }
    pub fn rand(rows: usize, cols: usize) -> Matrix<T>{
        random_matrix(rows, cols)
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    pub fn t(&self) -> Matrix<T> {
        let mut count = 0;
        let mut count2 = 0;
        let mut offset = 0;
        let addition = self.cols;

        let mut vec2: Vec<T> = Vec::new();

        while offset < self.cols {

            let i = self.data[count+offset];

            count += addition;
            count2 +=1;
            if count2 >= self.rows {
                count2 = 0;
                count = 0;
                offset +=1;
            }
            vec2.push(i);
        }
        Matrix {rows: self.cols, cols: self.rows, data: vec2}
    }

    pub fn rdot(&self, mx: &Matrix<T>) -> Matrix<T> {
    
        let mx = &mx.t();
     //   let cols_mx = mx.clone().data;
        let mut cols_mx = self.data.clone();
        let mut result = T::number(0.);
        let mut vec: Vec<T> = Vec::new();
    //    let mut rows = mx.values.chunks(mx.cols);
   //     println!("cols {:?}", cols);
        let mut run = 0;
        let mut runtime = 1;

        while runtime <= self.rows {

            let col = cols_mx.split_at(self.cols);
            while run <= mx.rows-1 {
                //let z = rows.__iterator_get_unchecked(run);
                let z = mx.get_row(run);
                run +=1;
                for (x, z) in z.iter().enumerate().take(self.cols) {
                    result = T::add(result, T::mul(z.clone(),col.0[x]));
                }
                vec.push(result);
                result = T::number(0.);
            }      
            cols_mx = col.1.to_vec();
            run = 0;
            runtime+=1;

        }
     //      
        Matrix {rows: self.rows, cols: mx.rows, data: vec}
           
    }
    pub fn max(&self, max: T) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for data in self.data.clone() {
            if T::comp(data, max).unwrap() == std::cmp::Ordering::Less {
                vec.push(max.clone());
            } else {
                vec.push(data);
            }
        }
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }
    pub fn add(&self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for (i, x) in self.data.iter().enumerate().take(self.cols*self.rows) {
            vec.push(T::add(*x, rhs.data[i]))
        } 
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }
    pub fn mul(&self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for (i, x) in self.data.iter().enumerate().take(self.cols*self.rows) {
            vec.push(T::mul(*x, rhs.data[i]))
        } 
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }
    pub fn sub(&self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for (i, x) in self.data.iter().enumerate().take(self.cols*self.rows) {
            vec.push(T::sub(*x, rhs.data[i]))
        } 
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }
    pub fn div(&self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for (i, x) in self.data.iter().enumerate().take(self.cols*self.rows) {
            vec.push(T::div(*x, rhs.data[i]))
        } 
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }
    pub fn sum(&self) -> T {   

        let mut sum = T::number(0.);
        let d = self.data.clone();
        if d.len() <= 1000 { //1000
            for d in d {
                
                sum = T::add(d, sum); //T::sum(d, sum);
      //          println!("sum value: {:?}", sum);
            }
            return sum;
        }
        let data = d.split_at(self.cols*self.rows/2);

        let data1: Vec<T> = data.0.to_vec();
        let data2:Vec<T> = data.1.to_vec();

        let i = th_s(data2);
        let x = th_s(data1);
        T::add(i.join().unwrap(),x.join().unwrap())
    }
    fn get_row(&self, index: usize) -> &[T] {
        let index = index*self.cols;
        &self.data[index..index+self.cols]
        
    }
    pub fn map<F>(&self, f: F) -> Matrix<T>
        where F: FnMut(&T) -> T {
            let data: Vec<T> = self.data.iter().map(f).collect();
            Matrix {rows: self.rows, cols: self.cols, data}
    }
 
}

/*
impl <T: Number + Clone>std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut vec: Vec<T> = Vec::new();
        for (i, x) in self.data.iter().enumerate().take(self.cols*self.rows) {
            vec.push(T::add(rhs.data[i].clone(), x.clone()));
        }
        Matrix {cols: self.cols, rows: self.rows, data: vec}
    }
}
*/


#[allow(dead_code)]
#[allow(unused)]
#[cfg(test)]
mod tests {
    use std::{time::{Duration, Instant}};
    use super::Matrix;

    fn performance() -> Duration {

        let matrix = Matrix::new(1000, 1000, 4f32);
        let matrix2 = Matrix::new(1000, 1000, 5f32);
    
        let time_before = Instant::now();
        //let row = matrix.get_col(100);
        
    //   matrix2.sum();
    //   matrix.sum();
        let time_after = Instant::now();
        return time_after-time_before;

    }

    fn test() {   
        let iter = 10000;
        let mut duration = 0.;
        for x in 0..iter {
            let dur: Duration = performance();
            duration += dur.as_secs_f32();
        }
        println!("Average duration (in secs): {}", duration/iter as f32);
    }

    fn check() {

        let a = Matrix::new(3, 3, 9.);
        let b = Matrix::new(3, 19, 51.);

        let c = a.rdot(&b);
        println!("c: {:?}", c);
        println!("len: {:?}", c.data.len());
    }

    fn main() {
    // test();
        //check();
        let m: Matrix<f32> = Matrix::rand(5000, 300);

        let vec1: Vec<f32> = vec![1., 3., 4., 5.,
                                6., 8., 9., 10.,
                                1., 3., 4., 5.,];

        let vec2: Vec<f32> = vec![1., 2., 3., 
                                6., 7., 8.,
                                1., 2., 3.,
                                6., 3., 5.,];

        let matrix = Matrix::from_vec(3, 4, vec1);
        let matrix2 = Matrix::from_vec(4, 3, vec2);

        
    // println!("matrix sum: {:?}", matrix.sum());

        //let m: Matrix<f32> = Matrix::rand(13, 510);

    //    println!("matrix rand: {:?}", m);
        
    // matrix.rdot(&matrix2);
        
    // println!("matrix dot: {:?}", matrix.dot(matrix2));


    }
}