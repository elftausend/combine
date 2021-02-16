use super::activation::ActivationType;
use crate::backend::matrix::Matrix;

pub trait Module<T>: std::fmt::Debug + Send {
    fn forward(&mut self, mxf: Matrix<T>) -> Matrix<T>;
}


#[derive(Debug, Clone)]
pub struct ModuleType<T> {
    pub activation: super::activation::ActivationType,
    pub matrix: Matrix<T>,
    pub bias: T, //matrix
}

impl <T: 'static>ModuleType<T>
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T>+ std::iter::Sum<&'static T> + crate::backend::traits::Number
    {
    pub fn new(activation: ActivationType, input: usize, output: usize) -> ModuleType<T> {
        let mx = Matrix::rand(input, output);
        ModuleType {activation, matrix: mx, bias: T::number(0.)}
    }
    pub fn from_vector(activation: ActivationType, mx: Matrix<T>) -> ModuleType<T> {
        ModuleType {activation, matrix: mx, bias: T::number(0.)}
    }
}

/*
impl <T: 'static>Optimizer<T> for ModuleType<T>
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T>+ std::iter::Sum<&'static T>
    {
    fn backwards(&mut self, _mxb: Matrix<T>) {
        let adjust = self.clone().matrix.map(|x| x+T::convert_to_format(23.));
   //     if mxb.sum() >
         self.matrix = self.matrix.clone().map(|x| x+adjust.clone().sum());
    }
}
*/

impl <T: 'static>Module<T> for ModuleType<T> 
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T>+ std::iter::Sum<&'static T>
    {
    fn forward(&mut self, mxf: Matrix<T>) -> Matrix<T> {
        mxf.rdot(self.matrix.clone())

 //       let x = mxf.multiply(self.matrix.clone()).adde(self.bias);
  //      self.matrix = x.clone();
  //      return x;

  //      self.matrix = mxf.adde(self.bias); ganz alt
  //      return self.matrix.clone();
    }
} 