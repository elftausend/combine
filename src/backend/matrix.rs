use std::thread;
use crate::backend::Math;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}
impl <T: 'static>Matrix<T> 
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + super::traits::Random +
        super::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T>
    {
    pub fn new(rows: usize, cols: usize, filling_data: T) -> Matrix<T> {
        Matrix {rows, cols, data: vec![filling_data; cols*rows]}
    }
    pub fn rand(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {rows, cols, data: vec![Default::default(); rows*cols]}.randomize()
    }
    pub fn from_vector(rows: usize, cols: usize, vec: Vec<T>) -> Matrix<T> {
        if vec.len() == rows*cols {
            Matrix {rows, cols, data: vec}
        } else {
            panic!("Vector lenght is {}, however rows times columns is {1}.. (Need to be {} too.)", vec.len(), rows*cols);
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    pub fn randomize(&self) -> Matrix<T> {
        let mut vec:Vec<T> = Vec::new();

        if self.data.len() <= 560 { //560
            for _x in 0..self.data.len() {
                vec.push(T::create_random());
            }
            return Matrix {rows: self.rows, cols: self.cols, data: vec};  
        }

        let t = (th_r((self.data.len() as f32/2.).ceil() as usize, ), th_r((self.data.len() as f32/2.).floor() as usize,));
        let mut x = t.0.join().unwrap();
        let mut x1 = t.1.join().unwrap();
        vec.append(&mut x);
        vec.append(&mut x1);
        if vec.len() != self.cols*self.rows {
            panic!("Error in randomizing.. Vector len: {0}, needed len: {1}", vec.len(), self.cols*self.rows);
        }
//73978
        Matrix {rows: self.rows, cols: self.cols, data: vec}

    }
   // #[inline]
    pub fn map<F>(&self, f: F) -> Matrix<T>
      where F: FnMut(T) -> T {

        let data: Vec<T> = self.data.clone().into_iter().map(f).collect();
        Matrix {rows: self.rows, cols: self.cols, data}
    }
    pub fn max(&self, max: T) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for data in self.data.clone() {
            if data < max {
                vec.push(max.clone());
            } else {
                vec.push(data);
            }
        }
        Matrix {rows: self.rows, cols: self.cols, data: vec}
    }

    pub fn sum(self) -> T {   

        let mut sum = 0.;
        let d = self.data.clone();
        if d.len() <= 1000 {
            for d in d {
                
                sum = Math::sum(d, sum); //T::sum(d, sum);
      //          println!("sum value: {:?}", sum);
            }
            return T::convert_to_format(sum);
        }
        let data = d.split_at(self.cols*self.rows/2);

        let data1: Vec<T> = data.0.to_vec();
        let data2:Vec<T> = data.1.to_vec();

        let i = th_s(data2);
        let x = th_s(data1);
        let sum = i.join().unwrap()+x.join().unwrap();
        sum 
    }
    #[inline]
    pub fn t(&self) -> Matrix<T> {
        let mut count = 0;
        let mut count2 = 0;
        let mut offset = 0;
        let addition = self.cols;

        let mut vec2: Vec<T> = Vec::new();

        while offset < self.cols {

            let i = &self.data[count+offset];

            count += addition;
            count2 +=1;
            if count2 >= self.rows {
                count2 = 0;
                count = 0;
                offset +=1;
            }
            vec2.push(i.clone());
        }
        Matrix {rows: self.cols, cols: self.rows, data: vec2}
    }
    #[inline]
    pub fn get_matrix(&self) -> Matrix<T> {
        Matrix {rows: self.rows, cols: self.cols, data: self.data.clone()}
    }
    #[inline]
    pub fn rdot(&self, mut mx: Matrix<T>) -> Matrix<T> {
        if self.shape().1 != mx.shape().0 {
            panic!("Cannot multiply {0}x{1} with {2}x{3}", self.shape().0, self.shape().1, mx.shape().0, mx.shape().1);
        }

        mx = mx.t();
     //   let cols_mx = mx.clone().data;
        let mut cols_mx = self.data.clone();
        let mut result = Default::default();
        let mut vec: Vec<T> = Vec::new();
        let mut rows = mx.data.chunks(mx.cols);
   //     println!("cols {:?}", cols);
        let mut run = 0;
        let mut runtime = 1;

        while runtime <= self.rows {

            let col = cols_mx.split_at(self.cols);
            unsafe {
                while run <= mx.rows {
                    let z = rows.__iterator_get_unchecked(run);
               //     let z = rows.get_unchecked(run);
                    if z.is_empty() {
                        break;
                    }
                    run +=1;
                    for (x, z) in z.iter().enumerate().take(self.cols) {
                        result = result + (z.clone()*col.0[x].clone());
                    }
                    /*
                    for x in 0..self.cols {
                        result = result + (z[x].clone()*col.0[x].clone());
                    }
                    */
                    vec.push(result);
                    result = Default::default();
                }
            }
            cols_mx = col.1.to_vec();
            run = 0;
            runtime+=1;

        }
     //      
        Matrix {rows: self.rows, cols: mx.rows, data: vec}
           
    }

    pub fn sub(self, other: Matrix<T>) -> Matrix<T> {
        if self.shape() != other.shape() {
            panic!("Both matrices have to have the same shape! {:?}, {:?}", self.shape(), other.shape())
        }
        let mut addend: Vec<T> = Vec::new();
        let other_data = other.data; 
        let mut subtrahend = other_data.into_iter();
        for minuend in self.data {

            let subtrahend = subtrahend.next();
            addend.push(minuend-subtrahend.unwrap());
            
        }
        Matrix::from_vector(self.rows, self.cols, addend)
    }

    pub fn div(self, other: Matrix<T>) -> Matrix<T> {
        if self.shape() != other.shape() {
            panic!("Both matrices have to have the same shape! {:?}, {:?}", self.shape(), other.shape())
        }
        let mut addend: Vec<T> = Vec::new();
        let other_data = other.data; 
        let mut subtrahend = other_data.into_iter();
        for minuend in self.data {

            let subtrahend = subtrahend.next();
            addend.push(minuend/subtrahend.unwrap());
            
        }
        Matrix::from_vector(self.rows, self.cols, addend)
    }
    pub fn add(self, other: Matrix<T>) -> Matrix<T> {
        if self.shape() != other.shape() {
            panic!("Both matrices have to have the same shape!")
        }
        let mut addend: Vec<T> = Vec::new();
        let other_data = other.data; 
        let mut summand2 = other_data.into_iter();
        for summand1 in self.data {

            let summand2 = summand2.next();
            addend.push(summand1+summand2.unwrap());
            
        }

        Matrix::from_vector(self.rows, self.cols, addend)
    }

    ///Only checks 1000 elements on equality.
    pub fn eq(&self, matrix: &Matrix<T>) -> bool {
        //   println!("------------------------------------------------"); 
        //   println!("[Matrix] Only checks 1000 elements on equality.");
        //   println!("------------------------------------------------");
           let mut check_value = 0;
           if self.shape() == matrix.shape() {
               for x in self.data.clone() {
                   for z in matrix.data.clone() {
                       check_value += 1;
                       if check_value == 1000 {
                           return true;
                       }
                       if x != z {
                           return false;
                       }
                   }
                   
               }
                true
           } else {
                false
           }
        }
    //Elementwise mulitplication
    pub fn mul(self, other: Matrix<T>) -> Matrix<T> {
        if self.shape() != other.shape() {
            panic!("Both matrices have to have the same shape! {:?}, {:?}", self.shape(), other.shape())
        }
        let mut value: Vec<T> = Vec::new();
        let other_data = other.data; 
        let mut subtrahend = other_data.into_iter();
        for minuend in self.data {

            let subtrahend = subtrahend.next();
            value.push(minuend*subtrahend.unwrap());
            
        }
        Matrix::from_vector(self.rows, self.cols, value)
    }
    
}

pub fn th_r<T: 'static>(elements: usize) -> std::thread::JoinHandle<Vec<T>> where T: super::traits::Random + Send + std::fmt::Debug {
    let builder = thread::Builder::new();
    builder.spawn(move || {
        let mut vec: Vec<T> = Vec::new();
        
        for _x in 0..elements {
            vec.push(T::create_random());
        }
        vec 
    }).unwrap()
}

pub fn th_s<T: 'static>(data2: Vec<T>) -> std::thread::JoinHandle<T> where T: std::ops::Add + Send + Default + std::ops::Add<Output = T> + crate::backend::traits::Math {
    let builder2 = thread::Builder::new();
    builder2.spawn(move || {
        let mut x = 0.;
        for d in data2 {
            x = T::sum(d, x);
        }
        T::convert_to_format(x)
    }).unwrap()
}




