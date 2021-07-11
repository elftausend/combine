
pub mod structual;
pub mod storing;
pub mod backend;
pub use backend::*;
pub use storing::*;
pub use structual::*;

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    
    use crate::ActivationType;

    use super::Matrix;
    use super::structual::{Activation, ActivationType::{Sigmoid, Relu, None}, ModuleType, Network};
    use std::time::{Duration, Instant};
    #[test]
    fn net() {
        let mut nn: Network<f32> = Network::new();
        nn.add(ModuleType::new(ActivationType::Relu, 110, 800));
        nn.add(ModuleType::new(ActivationType::Relu, 800, 100));
        nn.add(ModuleType::new(ActivationType::Relu, 100, 300));

        let forward = Matrix::rand(1, 110);

        nn.forward(forward.clone());

        for _ in 0..1000 {
            nn.forward(forward.clone());
        }
    }
    #[test]
    fn net1() {
        let xf = vec![0., 0.,
        1., 0.,
        0., 1.,
        1., 1.,];

        //desired output
        //XOR; 4x1 
        let yf = vec![0.,
            1.,
            1.,
            0.];




        let x: Matrix<f32> = Matrix::from_vec(4, 2, xf);
        let y: Matrix<f32> = Matrix::from_vec(4, 1, yf);

        let mut nn: Network<f32> = Network::new();
        nn.add(ModuleType::new(ActivationType::Relu, 2, 4000));
        nn.add(ModuleType::new(ActivationType::Relu, 4000, 4000));
        //nn.add(ModuleType::new(ActivationType::Relu, 4000, 4000));
        //nn.add(ModuleType::new(ActivationType::Relu, 4000, 4000));
        nn.add(ModuleType::new(ActivationType::Relu, 4000, 4));
        nn.add(ModuleType::new(ActivationType::Relu, 4, 1));

        let forward = Matrix::rand(4, 2);

        nn.forward(forward.clone());

        for _ in 0..300 {
            nn.forward(forward.clone());
            nn.backwards(forward.clone());
            //println!("x");
        }
    }
    #[test]
    fn size() {
        let m1: Matrix<f32> = Matrix::new(10000, 10000, 0f32);
        let m2: Matrix<f32> = Matrix::new(10000, 10000, 0f32);

        for _ in 0..1000 {
            m1.add(&m2);    
        }
        
    }
    #[test]
    fn xor() {
        let xf = vec![0., 0.,
        1., 0.,
        0., 1.,
        1., 1.,];

        //desired output
        //XOR; 4x1 
        let yf = vec![0.,
            1.,
            1.,
            0.];




        let x: Matrix<f32> = Matrix::from_vec(4, 2, xf);
        let y: Matrix<f32> = Matrix::from_vec(4, 1, yf);

        let mut nn = Network::new(); //in Zusammenhang von f16; das ist auch f32!!
        nn.learning_rate(7.).aiming(&y);

        nn.add(ModuleType::new(Sigmoid, 2, 4));
   //    nn.add(ModuleType::new(Sigmoid, 2, 4));
        nn.add(ModuleType::new(Sigmoid, 4, 1));

        // let dots = nn.clone().dots;
        // let aiming = nn.clone().aiming;




        let _time_before = Instant::now();

        let o = nn.forward(x.clone());
        println!("predictions before training: {:?}", o);

        for z in 0..5000 { //230000
            let cost = nn.forward_mse(x.clone(), y.clone()).sum();
            nn.backwards(x.clone());
        //   println!("cost: {:?}", cost/*.to_f32()*/);
          //  if z%20000 == 1 { //1300, 820
            //    println!("{:?}", cost/*.to_decimal()*/);
          //  }
        }
        let o = nn.forward(x.clone());
        println!("predictions after training: {:?}", o);
    }
    #[test]
    fn it_works() {

        let mut nn: Network<f32> = Network::new();
        nn.aiming(&Matrix::rand(128*2, 10*2)).learning_rate(7f32);
        let rand: Matrix<f32> = Matrix::rand(128*2, 784*2);
        nn.add(ModuleType::new(ActivationType::Sigmoid, 784*2, 128*2));
        nn.add(ModuleType::new(None, 128*2, 10*2));
        for _ in 0..1000 {
            nn.backwards(rand.to_owned());
        }
        
        //std::process::exit(0);

        /* 
        //A B
        let xf = vec![0., 0.,
        1., 0.,
        0., 1.,
        1., 1.,];

        //desired output
        //XOR; 4x1 
        let yf = vec![0.,
            1.,
            1.,
            0.];




        let x: Matrix<f32> = Matrix::from_vec(4, 2, xf);
        let y: Matrix<f32> = Matrix::from_vec(4, 1, yf);

        let mut nn = Network::new(); //in Zusammenhang von f16; das ist auch f32!!
        nn.learning_rate(7.).aiming(&y);

        nn.add(ModuleType::new(Sigmoid, 2, 4));
   //    nn.add(ModuleType::new(Sigmoid, 2, 4));
        nn.add(ModuleType::new(Sigmoid, 4, 1));

        // let dots = nn.clone().dots;
        // let aiming = nn.clone().aiming;




        let _time_before = Instant::now();

        let o = nn.forward(x.clone());
        println!("predictions before training: {:?}", o);

        for z in 0..100 { //230000
            //let cost = nn.forward_mse(x.clone(), y.clone()).sum();
            nn.backwards(x.clone());
        //   println!("cost: {:?}", cost/*.to_f32()*/);
          //  if z%20000 == 1 { //1300, 820
          //      println!("{:?}", cost/*.to_decimal()*/);
          //  }
        }
        println!("preds after training:");
        let o = nn.forward(x);
        for data in o.data {
            println!("{:?}", data/*.to_f32()*/);
        }

        println!("-----------------------------");


        let xf = vec![1., 0.,
                    1., 1.,
                    0., 1.,
                    1., 0.,];

        let x: Matrix<f32> = Matrix::from_vec(4, 2, xf);

        let o = nn.forward(x);
        println!("with new input: ");
        for data in o.data {
            println!("{:?}", data/*.to_f32()*/);
        }



*/
    }
}
