
pub mod structual;
pub mod storing;
pub mod backend;
pub use backend::*;
pub use storing::*;
pub use structual::*;

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    
    use super::Matrix;
    use super::structual::{Activation, ActivationType::{Sigmoid, Relu, None}, ModuleType, Network};
    use std::time::{Duration, Instant};
    #[test]
    fn it_works() {
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

        for z in 0..3400_000 { //230000
            let cost = nn.forward_mse(x.clone(), y.clone()).sum();
            nn.backwards(x.clone());
        //   println!("cost: {:?}", cost/*.to_f32()*/);
            if z%10000 == 1 { //1300, 820
                println!("{:?}", cost/*.to_decimal()*/);
            }
        }
        println!("preds after training:");
        let o = nn.forward(x);
        for data in o.data {
            println!("{:?}", data/*.to_f32()*/);
        }


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




    }
}
