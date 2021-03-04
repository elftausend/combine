use crate::{Matrix, Number};

pub trait Activation<T>: std::fmt::Debug {

    fn sigmoid(&self, mx: &Matrix<T>) -> Matrix<T>;
    fn sigmoid_derivative(&self, mx: &Matrix<T>) -> Matrix<T>;
    fn relu(&self, mx: &Matrix<T>) -> Matrix<T>;
}

#[derive(Debug, Clone)]
pub enum ActivationType {
    Sigmoid,
    Relu,
    None,
    
}

impl <T>Activation<T> for ActivationType
    where T: std::fmt::Debug + Clone + Copy + Number + Send + 'static {
    fn sigmoid(&self, mx: &Matrix<T>) -> Matrix<T> {
        //    return mx.negate().exp().adde(1.).dividef(1.0.into());
       //     return mx.sigmoid();
            mx.map(|x| T::div(T::number(1.),T::add(T::exp(T::mul(*x, T::number(-1.))), T::number(1.))))
        }
        fn sigmoid_derivative(&self, mx: &Matrix<T>) -> Matrix<T> {
          //  return mx.negate().exp()/(mx.negate().exp().adde(1.).pow(2));
       //     return mx.sigmoid_derivative();
            mx.map(|x| T::div(T::exp(T::mul(*x, T::number(-1.))),T::pow(T::add(T::exp(T::mul(*x, T::number(-1.))), T::number(1.)), T::number(2.))))
         //   return mx.map(|x| (-x).exp()/((-x).exp()+1.).powi(2));
        }
        fn relu(&self, mx: &Matrix<T>) -> Matrix<T> {
            mx.max(T::number(0.))
        }
}