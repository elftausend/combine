#![feature(trusted_random_access)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub mod backend;
pub mod structual;
pub mod nn;
pub mod storing;
use std::time::Duration;
pub use structual::*;
pub use backend::matrix::Matrix;
pub use backend::datatypes::*;
pub use backend::traits::Random;
pub use nn::Network;
pub use crate::structual::ActivationType::*;
pub use crate::structual::{Optimizer, Module};
pub use storing::WeightsStore;

use std::time::Instant;

fn mapping<T: 'static>() where T: Default + std::fmt::Debug + Send + Clone +
std::ops::Add + std::ops::Add<Output = T> + std::cmp::PartialOrd +
std::ops::Mul<Output = T> + std::ops::Add<<T as std::ops::Mul>::Output> + 
std::ops::Add<<T as std::ops::Mul>::Output, Output = T> + backend::traits::Random
  + backend::traits::Math + std::ops::Div<Output = T> + std::ops::Neg<Output = T> +
  std::ops::Sub<Output = T> + std::iter::Sum<&'static T>{

 // use std::time::Instant;
  let before = Instant::now();
  let x: Matrix<T> = Matrix::rand(3, 3);
  for _x in 0..1000000 {
   
    //  let x: Matrix<XES> = Matrix::new(3, 3, XES::new(1));
      x.map(|x| T::convert_to_format(1.)/(T::exp(-x)+T::convert_to_format(1.)));
  }

  let after = Instant::now();

  println!("mapped: {:?}", after-before);

}



fn network_test() {

  let x: Vec<UES> = vec![UES::new(0.), UES::new(0.),
  UES::new(1.), UES::new(0.),
  UES::new(0.), UES::new(1.),
  UES::new(1.), UES::new(1.),];


//desired output
//XOR; 4x1
let y: Vec<UES> = vec![UES::new(0.),
  UES::new(1.),
  UES::new(1.),
  UES::new(0.)];





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


  

    let x: Matrix<f32> = Matrix::from_vector(4, 2, xf);
    let y: Matrix<f32> = Matrix::from_vector(4, 1, yf);

    let mut nn = Network::new(); //in Zusammenhang von f16; das ist auch f32!!
    nn.learning_rate(7.).aiming(y.clone());

    nn.add(structual::ModuleType::new(Sigmoid, 2, 4));
    //nn.add(structual::ModuleType::new(Sigmoid, 2, 4));
    nn.add(structual::ModuleType::new(Sigmoid, 4, 1));

    // let dots = nn.clone().dots;
    // let aiming = nn.clone().aiming;




    let time_before = Instant::now();

    let o = nn.forward(x.clone());
    println!("predictions before training: {:?}", o);

    for z in 0..100_000 { //230000
        let cost = nn.forward_mse(x.clone(), y.clone()).sum();
        nn.backwards(x.get_matrix());
     //   println!("cost: {:?}", cost/*.to_f32()*/);
        if z%10000 == 1 { //1300, 820
          println!("{:?}", cost/*.to_decimal()*/);
        }
        /*
        if cost <= 0.001/* XES::new(12) */{ //0.00005
            println!("Optimum reached!");
            break;
        }
        
*/
        }
    println!("preds after training:");
    let o = nn.forward(x);
    for data in o.data {
      println!("{:?}", data/*.to_f32()*/);
    }




    println!("nn before saving: {:?}", nn);

    WeightsStore::new().save(nn.clone(), "./structure/");

    let mut nn = WeightsStore::new().construct("./structure/");

    println!("-----------------------------------");

    // nn.aiming = aiming;
    // nn.dots = dots;

    println!("after saving nn: {:?}", nn);

    let xf = vec![1., 0.,
                          1., 1.,
                          0., 1.,
                          1., 0.,];

    let x: Matrix<f32> = Matrix::from_vector(4, 2, xf);

    let o = nn.forward(x);
    println!("");
    for data in o.data {
      println!("{:?}", data/*.to_f32()*/);
    }

    let time_after = Instant::now();
    println!("Duration: {:?}", time_after-time_before);
}
//#[inline]
fn performance() -> Duration {
  let vec1 = vec![1., 2.,
                  3., 4.,
                  5., 6.,];
  let vec2 = vec![7., 8., 9.,
                  1., 2., 3.,];

  let m: Matrix<f32> = Matrix::from_vector(3, 2, vec1);
  let m2: Matrix<f32> = Matrix::from_vector(2, 3, vec2);

  let a = Matrix::new(500, 100, 9f32);
  let b = Matrix::new(100, 500, 51f32);
  

  let a: f64 = 14.524358972938472983423;
  let b: f64 = 18.912307492349493332228;

  let time_before = Instant::now();
 // let c = a.rdot(b);
  //b.sum();
  let x = a*b;

  let time_after = Instant::now();
  return time_after-time_before;
  //println!("Duration: {:?}", time_after-time_before);
}
//0.0000156
#[test]
fn test() {
    network_test();

  //0.000000005336447  f32
  //0.000000005326992  f64



    let af64: f64 = 4314232.43244233213247;

    //0000000100100100000000000100000000000000000000010101010101010
    //00000001001001000000000000000000000000
    let af32: f32 = 4314232.43244233213247;

   // println!("f64: {}", af64);
   // println!("f32: {}", af32);
    //let matrix: Matrix<f32> = Matrix::rand(10, 10);

    //println!("matrix: {:?}", matrix);
    
    let iter = 100000;
    let mut duration = 0.;
    for x in 0..iter {
      let dur: Duration = performance();
      duration += dur.as_secs_f32();
    }
    println!("Average duration (in secs): {}", duration/iter as f32);
   // 0.035337962
  //network_test();
  // mapping::<XES>();
 // test1();
  /*

    let x = XES::new(10);
  //  println!("x: {:?}", x.create_random());

    

    let m = Matrix::new(3, 3, 3_f32);
    let m1 = Matrix::new(3, 3, 9_f32);

    let m3 = m.rdot(m1);
 //   println!("m3 {:?}", m3);

    let ab = vec![54., 61., 4., 1., 7.,
                  54., 61., 4., 1., 7.];



    let ac = vec![513., 341.,
                 214., 7.,];

    let au: Matrix<f32> = Matrix::from_vector(2, 5, ab);
    let ac: Matrix<f32> = Matrix::from_vector(2, 2, ac);

  //  println!("uc {:?}", au.t().rdot(ac));

    use std::time::Instant;
    let time_before = Instant::now();
    for x in 0..500000 {
  //      m1.rdot(m2.clone());
   //     let x: Matrix<f32> = Matrix::rand(5, 5);
    }
    let time_after = Instant::now();
    println!("Duration: {:?}", time_after-time_before);

    */
}
fn test1() {



  let ab = vec![12., 0.9, 4., 1.4, 7.,
                12., 0.9, 4., 1.4, 7.];



  let ac = vec![17., 13.,
                21., 7.,];

  let ab: Matrix<f32> = Matrix::from_vector(2, 5, ab);

  ab.map(|x| x+1.);

  let ac: Matrix<f32> = Matrix::from_vector(2, 2, ac);

 // println!("dot f32: {:?}", ab.t().rdot(ac));


  let ab = vec![XES::new(120), XES::new(91), XES::new(40), XES::new(141), XES::new(70),
                XES::new(120), XES::new(91), XES::new(40), XES::new(141), XES::new(70)];



  let ac = vec![XES::new(170), XES::new(130),
                XES::new(210), XES::new(70)];


  let ab: Matrix<XES> = Matrix::from_vector(2, 5, ab);
  let ac: Matrix<XES> = Matrix::from_vector(2, 2, ac);


 // println!("dot XES: {:?}", ab.t().rdot(ac));






  //  let x: Matrix<UES> = Matrix::rand(5, 6);
  //let x = UES::new(0.);
   // println!("x: {:?}", UES::create_random());
     let sum = UES::new(0.);
     
  //   let ues = Matrix::new(21, 43, XES::new(441));

    // println!("sum {:?}", ues.sum());
  //  let ues = Matrix::new(1, 41, UES::new(4.4));
  //  let f32 = Matrix::new(1, 41, 4.4);

 //   println!("ues sum: {:?}", ues.sum());
  //  println!("f32 sum: {:?}", f32.sum());

   // let x: Matrix<f32> = Matrix::rand(10, 70);
   let x = Matrix::new(10, 2, 0.00012_f32);
   let x1 = Matrix::new(2, 10, 0.12_f32);

  // println!("dot f32: {:?}", x.rdot(x1));
   let x = Matrix::new(10, 2, XES::new(125));
   let x1 = Matrix::new(2, 10, XES::new(122));
 //  println!("dot XES: {:?}", x.rdot(x1));

   let x = Matrix::new(10, 2, UES::new(0.00012));
   let x1 = Matrix::new(2, 10, UES::new(0.12));
  // println!("dot UES: {:?}", x.rdot(x1));

    let i: i16 = 9999; 

 //  backend::xes::testxes() //1008 -> bug -> to_xes_format()!! [transformation von 100.8 zu xes];
/*
    let m1 = Matrix::new(3, 3, 3_f32);
    let m2 = Matrix::new(3, 3, 9_f32);
 //   println!("matrix f32: {:?}", m1);
      println!("matrix f32 with add: {:?}", m1.adde(10.));

    println!("");

    let m = Matrix::new(3, 3, XES::new(30));
    println!("matrix xes: {:?}", m);
    println!("matrix with add: {:?}", m.adde(XES::new(100)));

    println!("");

    let m = Matrix::new(3, 3, XES::new(30));
    println!("matrix xes: {:?}", m);
    println!("matrix with add: {:?}", m.map(|x| x.pow(4.)));


    */
}
