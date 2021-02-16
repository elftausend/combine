use crate::backend::matrix::Matrix;

pub trait Optimizer<T>: std::fmt::Debug + Send {
    fn backwards(&mut self, mxb: Matrix<T>);
}