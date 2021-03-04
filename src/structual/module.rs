use crate::{Matrix, Number};
use crate::structual::ActivationType;


#[derive(Debug, Clone)]
pub struct ModuleType<T> {
    pub activation: ActivationType,
    pub matrix: Matrix<T>,
    pub bias: T,
}

impl <T>ModuleType<T> where T: Number + Clone + Copy + std::fmt::Debug + Send + 'static {
    pub fn new(activation: ActivationType, input: usize, output: usize) -> ModuleType<T> {
        let mx = Matrix::rand(input, output);
        ModuleType {activation, matrix: mx, bias: T::number(0.)}
    }
    pub fn from_vector(activation: ActivationType, mx: Matrix<T>) -> ModuleType<T> {
        ModuleType {activation, matrix: mx, bias: T::number(0.)}
    }
    pub fn forward(&self, mxf: &Matrix<T>) -> Matrix<T> {
        mxf.rdot(&self.matrix)
    }
}