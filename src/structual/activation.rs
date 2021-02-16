use crate::backend::matrix::Matrix;

pub trait Activation<T>: std::fmt::Debug + Send {

    fn sigmoid(&self, mx: Matrix<T>) -> Matrix<T>;
    fn sigmoid_derivative(&self, mx: Matrix<T>) -> Matrix<T>;
    fn relu(&self, mx: Matrix<T>) -> Matrix<T>;
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ActivationType {
    Sigmoid,
    Relu,
    None,
}

impl <T: 'static>Activation<T> for ActivationType 
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> + 
        std::ops::Sub<Output = T>+ std::iter::Sum<&'static T>
    {
    fn sigmoid(&self, mx: Matrix<T>) -> Matrix<T> {
    //    return mx.negate().exp().adde(1.).dividef(1.0.into());
   //     return mx.sigmoid();
        mx.map(|x| T::convert_to_format(1.)/(T::exp(-x)+T::convert_to_format(1.)))
    }
    fn sigmoid_derivative(&self, mx: Matrix<T>) -> Matrix<T> {
      //  return mx.negate().exp()/(mx.negate().exp().adde(1.).pow(2));
   //     return mx.sigmoid_derivative();
        mx.map(|x| T::exp(-x.clone())/T::pow(T::exp(-x)+T::convert_to_format(1.), 2.))
     //   return mx.map(|x| (-x).exp()/((-x).exp()+1.).powi(2));
    }
    fn relu(&self, mx: Matrix<T>) -> Matrix<T> {
        mx.max(T::convert_to_format(0.))
    }  
}

