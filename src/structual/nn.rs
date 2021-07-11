use crate::{Matrix, Number};
use super::{ModuleType, ActivationType, Activation};

#[derive(Debug, Clone)]
pub struct Network<T> {
    pub weights: Vec<ModuleType<T>>,
    pub dots: Vec<Matrix<T>>,
    aiming: Matrix<T>,
    learning_rate: f32,
}

impl <T>Network<T> where T: std::fmt::Debug + Number + Send + Clone + Copy + 'static {
        
    pub fn forward(&mut self, mxf: Matrix<T>) -> Matrix<T> {
        let mut mltp_data = mxf;
        self.dots = Vec::new();
        for i in 0..self.weights.len() {
            let m = &mut self.weights[i];
            mltp_data = m.forward(&mltp_data);
            println!("forward done");
            //println!("pushed in self.dots: {:?}", mltp_data.clone());
            self.dots.push(mltp_data.clone());
            
            mltp_data = mltp_data.map(|x| T::add(*x, m.bias));            
            match &m.activation {
                ActivationType::Sigmoid => {mltp_data = m.activation.sigmoid(&mltp_data);}
                ActivationType::Relu => {mltp_data = m.activation.relu(&mltp_data);}
                _ => {}
            }
        }
   //     println!("pushed: {:?}", self.dots);
        mltp_data
    }
    //only sigmoid?!
    pub fn backwards(&mut self, mxf: Matrix<T>) { 
        println!("HI");
        let mut dcdw: Vec<Matrix<T>> = Vec::new();
        let forward = self.forward(mxf.clone());
        println!("forward done");
        let cost = self.aiming.sub(&forward).map(|x| T::mul(*x, T::number(2.)));
        let mut delta_data = cost.map(|x| T::mul(*x, T::number(-1.)));


        for x in (0..self.weights.len()).rev() {
            if x == 0 { //last layer:
                delta_data = (delta_data.clone().rdot(&self.weights[x+1].matrix.t())).mul(&ActivationType::Sigmoid.sigmoid_derivative(&self.dots[x]));
      //          println!("Test");
                dcdw.push(mxf.clone().t().rdot(&delta_data));
                continue;
            }
            if x == self.weights.len()-1 { //first layer:
                delta_data = (delta_data.clone()).mul(&ActivationType::Sigmoid.sigmoid_derivative(&self.dots[x]));
      //        println!("Test2");
                dcdw.push(ActivationType::Sigmoid.sigmoid(&self.dots[x-1]).t().rdot(&delta_data));
                continue;
            } //all other layers:
            delta_data = (delta_data.rdot(&self.weights.clone()[x+1].matrix.t())).mul(&ActivationType::Sigmoid.sigmoid_derivative(&self.dots[x]));
            dcdw.push(ActivationType::Sigmoid.sigmoid(&self.dots[x-1]).t().rdot(&delta_data));

        }

        let mut vec: Vec<Matrix<T>> = Vec::new();
        for x in (0..dcdw.len()).rev() {
            vec.push(dcdw[x].clone());
        }
        for (x, vec) in vec.iter().enumerate().take(self.weights.clone().len()) {
            self.weights[x].matrix = self.weights[x].matrix.sub(&vec.map(|x| T::mul(*x,T::number(self.learning_rate))))
        }
    
    }
    pub fn new() -> Network<T> {
        Network {
            weights: Vec::new(), 
            dots: Vec::new(), 
            aiming: Matrix::new(0, 0, T::number(0.)), 
            learning_rate: 0.,
        }
    }
    pub fn learning_rate(&mut self, rate: f32) -> &mut Network<T> {
        self.learning_rate = rate;
        self
    }
    pub fn aiming(&mut self, aiming: &Matrix<T>) -> &mut Network<T> {
        self.aiming = aiming.clone();
        self
    }
    pub fn add(&mut self, m: ModuleType<T>) {
        self.weights.push(m);
    }
    ///Mean Squared Error Cost
    pub fn forward_mse(&mut self, mut mxf: Matrix<T>, mxt: Matrix<T>) -> Matrix<T> {
        mxf = self.forward(mxf);
        (mxf.sub(&mxt)).map(|x| T::pow(*x, T::number(2.)))
    }
    pub fn forward_test(&mut self, mut mxf: Matrix<T>, mxe: Matrix<T>) -> Matrix<T> {
        mxf = self.forward(mxf);
        mxe.sub(&mxf).mul(&ActivationType::Sigmoid.sigmoid_derivative(&mxf))
        
    }
}