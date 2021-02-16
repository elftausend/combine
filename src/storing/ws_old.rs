use crate::ActivationType;
use std::*;
use std::io::prelude::*;



use crate::nn::*;
use crate::Matrix;

pub struct WeightsStore2;

impl WeightsStore2 {
    pub fn new() -> WeightsStore2 {
        return WeightsStore2;
    }
    ///this will override (if exists) the old weights
    pub fn save<T>(&self, nn: Network<T>) {
     //   println!("{:?}", nn.weights);
        
        let mut x = Vec::new();
        
        write!(&mut x, "{:?}", nn.weights).unwrap();
        fs::write("./wstore.yml", x.clone()).unwrap();
    }
    pub fn construct<T>(&self) -> Network<T> {
        
        let weights = fs::read("./wstore.yml").unwrap();
        let mut weight_len = 0;
        let mut state = 0;
        let mut token = String::new();
        let mut save_token = String::new();
        let mut activation = vec![];
        let mut matrix = vec![];
        for d in weights.clone() {
            let c = char::from_u32(d as u32).unwrap();
            token.push(c);
            if state == 0 {
                if c == ' ' {
                    token = String::new();
                }
            } 
            if state == 1 {
                save_token.push(c);
                if c == ',' {
                    activation.push(save_token.clone());
                    state = 0;
                    save_token = String::new();
                }

            }
            if state == 2 {
                save_token.push(c);
                if c == '}' {
                    matrix.push(save_token.clone());
                    state = 0;
                    save_token = String::new();

                }
            }

            if token == "[ModuleType" || token == "ModuleType" {
                println!("Found ModuleType");
                weight_len += 1;
                token = String::new();
            }
            if token == "activation:" {
                println!("Found Activation");
                token = String::new();
                state = 1;
            }
            if token == "matrix:" {
                println!("Found Matrix");
                token = String::new();
                state = 2;
            }
            if token == "bias:" {
                println!("Found Bias");
                token = String::new();
                state = 0;
            }
         //   println!("{}", token);
               
        //    println!("Data: {0}, char: {1:?}", d, c);

        }
        let nn = matrix_lex(matrix.clone(), activation, weight_len);
        //println!("{:?}", activation);
       // println!("");
      //  println!("{:?}", matrix);
        return nn;
    }
}

fn matrix_lex<T>(data: Vec<String>, act: Vec<String>, wl: i32) -> Network<T> {
    let mut network = Network::new(Matrix::from_vector(0, 0, Vec::new()));
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut data_vec: Vec<f32> = Vec::new();

    let mut state = 0;
    let mut tokens = String::new();

    let mut rows_string = String::new();
    let mut cols_string = String::new();
    let mut data_string = String::new();
    for epoch in 0..wl {
        let data = &data[epoch as usize];
     //   println!("data {:?}", data);
        for d in data.chars() {
            tokens.push(d);
            if state == 0 {
                if d == ' ' {
                    tokens = String::new();
                }
            }
            if state == 1 {
                if d != ',' {
                    rows_string.push(d);
                } else {
                    state = 0;
                }
            }
            if state == 2 {
                if d != ',' {
                    cols_string.push(d);
                } else {
                    state = 0;
                }
            }
            if state == 3 {
                if d == ']' {
                    state = 0;
                }
                data_string.push(d);

            }
            if tokens == "rows:" {
                tokens = String::new();
                state = 1;
            }
            if tokens == "cols:" {
                tokens = String::new();
                state = 2;
            }
            if tokens == "data:" {
                tokens = String::new();
                state = 3;
            }
            
        }
        rows_string = rows_string.chars().filter(|c| !c.is_whitespace()).collect();
        cols_string = cols_string.chars().filter(|c| !c.is_whitespace()).collect();
        rows = rows_string.parse().unwrap();
        cols = cols_string.parse().unwrap();
    //    println!("{}", data_string);
        data_vec = get_data(data_string);
        let activation = get_activation(act.clone(), epoch);
        let matrix = Matrix::from_vector(rows, cols, data_vec);
    //    println!("!atrix: {:?}", matrix);
        network.add(crate::structual::ModuleType {activation: activation, matrix: matrix, bias: 0.});
        rows = 0;
        cols = 0;
        data_vec = Vec::new();
        rows_string = String::new();
        cols_string = String::new();
        data_string = String::new();
        tokens = String::new();
        state = 0;
        
    }
    return network;

}
fn get_activation(act: Vec<String>, index: i32) -> ActivationType {
    let mut activaton = ActivationType::Relu;
    let x = &act[index as usize];
    let t: String = x.chars().filter(|c| c != &',').collect();
    let t: String = t.chars().filter(|c| !c.is_whitespace()).collect();
    if t == "Sigmoid" {
        activaton = ActivationType::Sigmoid;
    }
    if t == "Relu" {
        activaton = ActivationType::Relu;
    }
    if t == "None" {
        activaton = ActivationType::None;
    }
    return activaton;
}
fn get_data(data: String) -> Vec<f32> {
  //  println!("{}", data);
    let mut token = String::new();
    let mut real_vec: Vec<f32> = Vec::new();
    let mut string_putting = String::new();
    let mut float = 0.0;
    let mut state = 0;
    let x: String = data.chars().filter(|c| c != &'[').collect();
  //  println!("{}", x);
    for c in x.chars() {
        if state == 0 {
            token.push(c);
        }
        if state == 1 {
            if c != ',' && c != ']' {
                string_putting.push(c);
            } else {
                state = 0;
                string_putting = string_putting.chars().filter(|c| !c.is_whitespace()).collect();
                float = string_putting.parse().unwrap();
                real_vec.push(float);

                float = 0.0;
                string_putting = String::new();
            }
        }

        if token == " " {
            state = 1;
        }
    }
//    println!("{:?}", real_vec);
    
    return real_vec;
}