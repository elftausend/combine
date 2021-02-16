use crate::structual::{Module, Activation, ActivationType, ModuleType, Optimizer};
use crate::backend::matrix::Matrix;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Network<T> {
    pub weights: Vec<ModuleType<T>>,
    pub dots: Vec<Matrix<T>>,
    aiming: Matrix<T>,
    learning_rate: f32,
}


impl <T: 'static>Optimizer<T> for Network<T>
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T> + std::iter::Sum<&'static T>
    {
    fn backwards(&mut self, mxf: Matrix<T>) { 

        let mut dcdw: Vec<Matrix<T>> = Vec::new();
        let cost = self.aiming.clone().sub(self.forward(mxf.clone()));
   //     println!("cost {:?}", cost.clone());
        let mut delta_data = cost.map(|x| -x);

        
        //   let dcdw1 = self.weights[1].matrix.get_matrix()*(ActivationType::Sigmoid.sigmoid_derivative(self.dots[1].get_matrix()))*(cost.mule(2.));

        for x in (0..self.weights.len()).rev() {
            if x == 0 { //last layer:
                delta_data = (delta_data.clone().rdot(self.weights.clone()[x+1].matrix.get_matrix().t())).mul(ActivationType::Sigmoid.sigmoid_derivative(self.dots[x].get_matrix()));
      //          println!("Test");
                dcdw.push(mxf.t().rdot(delta_data.clone()));
                continue;
            }
            if x == self.weights.len()-1 { //first layer:
                delta_data = (delta_data.clone()).mul(ActivationType::Sigmoid.sigmoid_derivative(self.dots[x].get_matrix()));
      //        println!("Test2");
                dcdw.push(ActivationType::Sigmoid.sigmoid(self.dots[x-1].get_matrix()).t().rdot(delta_data.clone()));
                continue;
            } //all other layers:
            delta_data = (delta_data.clone().rdot(self.weights.clone()[x+1].matrix.get_matrix().t())).mul(ActivationType::Sigmoid.sigmoid_derivative(self.dots[x].get_matrix()));
            dcdw.push(ActivationType::Sigmoid.sigmoid(self.dots[x-1].get_matrix()).t().rdot(delta_data.clone()));

        }

        let mut vec: Vec<Matrix<T>> = Vec::new();
        for x in (0..dcdw.len()).rev() {
            vec.push(dcdw.clone()[x].get_matrix());
        }
  //      let cost = (self.forward_mse(mxf.clone(), self.aiming.clone())).sum();
        for (x, vec) in vec.iter().enumerate().take(self.weights.clone().len()) {
            self.weights[x].matrix = self.weights.clone()[x].matrix.get_matrix().sub(vec.map(|x| x*T::convert_to_format(self.learning_rate)))
        }
        /*
        for x in 0..self.weights.clone().len() {
       //     println!("{:?}", vec[x]);
            self.weights[x].matrix = self.weights.clone()[x].matrix.get_matrix().sub(vec[x].map(|x| x*T::convert_to_format(2.3))); //0.18, 0.88
        }
        */
        
     //   let cost1 = (self.forward_mse(mxf.clone(), self.aiming.clone())).sum();
   //     println!("pre cost: {:?}", cost);
    //    println!("after cost: {:?}", cost1);
    }
}

impl <T: 'static>Module<T> for Network<T> 
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T>+ std::iter::Sum<&'static T>
    {
    fn forward(&mut self, mxf: Matrix<T>) -> Matrix<T> {
        let mut mltp_data = mxf;
      //  println!("{}", self.dots.len());
        //let bias = self.bias.clone();
        self.dots = vec![];
        for i in 0..self.weights.len() {
        //for m in &mut self.weights {
            let m = &mut self.weights[i];
            mltp_data = m.forward(mltp_data);
            self.dots.push(mltp_data.clone());
            
            mltp_data = mltp_data.clone().map(|x| x+m.bias.clone());
       //     mltp_data = mltp_data.multiply(m.matrix.clone());
    //        println!("{}", count);
            
            match &m.activation {
                ActivationType::Sigmoid => {mltp_data = m.activation.sigmoid(mltp_data.clone());}
                ActivationType::Relu => {mltp_data = m.activation.relu(mltp_data.clone());}
                _ => {}
            }
    //        m.forward(mltp_data.clone());
        }
        mltp_data
    }
}
#[allow(dead_code)]
impl <T: 'static>Network<T>
    where T: Default + std::fmt::Debug + Send + Clone +
        std::ops::Add<Output = T> + std::cmp::PartialOrd +
        std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
        std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + crate::backend::traits::Random +
        crate::backend::traits::Math + std::ops::Neg<Output = T> + std::ops::Div<Output = T> +
        std::ops::Sub<Output = T> + std::iter::Sum<&'static T> + crate::backend::traits::Number
    {
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
    pub fn aiming(&mut self, aiming: Matrix<T>) -> &mut Network<T> {
        self.aiming = aiming;
        self
    }
    pub fn add(&mut self, m: ModuleType<T>) {
        self.weights.push(m);
  //      self.weights.push(Box::new(m));
        //<M: Module + 'static>
    }
    ///Mean Squared Error Cost
    pub fn forward_mse(&mut self, mut mxf: Matrix<T>, mxt: Matrix<T>) -> Matrix<T> {
        mxf = self.forward(mxf);
        //   let cost = (mxt.clone()-mxf.clone()).pow(2);
        (mxf.sub(mxt)).map(|x| T::pow(x, 2.))
    }
    pub fn forward_test(&mut self, mut mxf: Matrix<T>, mxe: Matrix<T>) -> Matrix<T> {
        mxf = self.forward(mxf);
        mxe.sub(mxf.clone()).mul(ActivationType::Sigmoid.sigmoid_derivative(mxf))
        
    }


}

