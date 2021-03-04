use crate::ActivationType;

use std::*;
use std::io::prelude::*;

use crate::Matrix;
use crate::nn::*;
use crate::structual::ModuleType;


pub struct WeightsStore;

impl WeightsStore {
    pub fn new() -> WeightsStore {
        return WeightsStore;
    }
    ///this will override (if exists) the old weights
    pub fn save(&self, nn: Network<f32>, path_str: &str) {
        fs::create_dir_all(&path_str).expect("error in dir creation");
        let mut paths: Vec<_> = fs::read_dir(path_str).unwrap()
            .map(|r| r.unwrap())
            .collect();

        paths.sort_by_key(|dir| dir.path());
        for path in paths {
            let path2 = &path;
      //      println!("{:?}", path);
            if path2.path().to_str() == Some(format!("{}.DS_Store", path_str).as_str()) {
                continue;
            }
            fs::remove_file(path.path()).unwrap();
        }
    
        let mut count = 0;
        
     //   println!("{:?}", nn.weights);
        for w in &nn.weights {
            

            let mut x = Vec::new();
            let shape = w.matrix.shape();
            let activation = &w.activation;
            write!(&mut x, "{:?}", w.matrix.clone().data).unwrap();
            let ccount = count+100;
            let path = format!("{}<{:?}>={:?}=#{:?}#.yml", path_str, ccount, shape, activation);
            println!("path: {:?}", path);
            fs::write(path, x.clone()).unwrap();
            count += 1;
         //   w.matrix.clone().data;
        }
  //      let mut x = Vec::new();
        
    //    write!(&mut x, "{:?}", nn.weights);
      //  fs::write("./wstore.yml", x.clone()).unwrap();
    }
    pub fn get_path_string(&self, paths: String, index: usize) {    
        let mut string_buffer: String = String::new();
        let mut state: usize = 0;
        let mut count: usize = 0;
        for x in paths.chars() {
            if x == '*' {
                if state == 0 {
                    state = 1;
                } else {
                    count += 1;
                    state = 0;
                }
            }
            if state == 1 {
                string_buffer.push(x);
            }
            if count != index {
                string_buffer = String::new();
            }
        }
    }
    pub fn construct2(&self) -> Network<f32> {
     //   let tasks: Vec<(usize, crate::structual::ModuleType<f32>)> = Vec::new(); 
        let mut nn = Network::new();
     //   let mut vec: String = String::new();
        let paths: Vec<_> = fs::read_dir("./structure/").unwrap().map(|r| r.unwrap()).collect();
    //    println!("{}", paths.len());
        crossbeam::scope(|scope| {
            let workers: Vec<_> = (0..paths.len()-1 as usize).map(|i| {
                    scope.spawn(move |_| {
      
                    let mut paths: Vec<_> = fs::read_dir("./structure/").unwrap().map(|r| r.unwrap()).collect();
                    paths.sort_by_key(|dir| dir.path());
                    paths.remove(0);
                        
                    let mut string: String = String::new();
                    let path = &paths[i];
                    let weights = fs::read(path.path()).unwrap();
                
                    for data in weights {
                        let c = char::from_u32(data as u32).unwrap();
                        string.push(c);
                    }
                    let vec = get_data(string.clone());
                    let shape = get_shape(path.path());
                    let activation = get_act(path.path());
                    return (i, ModuleType::from_vector(activation, Matrix::from_vec(shape.0, shape.1, vec)));
                

                    })
                })
                .collect();
        
        for worker in workers {
           let x = worker.join().unwrap();
           nn.add(x.1);
       //    tasks.push(x);      
      //     println!("{:?}", x);
        }
        }).unwrap();
        return nn;
    }

    //./structure/
    pub fn construct(&self, path_str: &str) -> Network<f32> {
        let mut nn = Network::new();
        let mut string = String::new();
        let mut paths: Vec<_> = fs::read_dir(path_str).unwrap()
            .map(|r| r.unwrap())
            .collect();

        paths.sort_by_key(|dir| dir.path());
        for path in paths {
            let path2 = &path;
      //      println!("{:?}", path);
            if path2.path().to_str() == Some(format!("{}.DS_Store", path_str).as_str()) {
                continue;
            }

            let weights = fs::read(path.path()).unwrap();
            for data in weights {
                let c = char::from_u32(data as u32).unwrap();
                string.push(c);
            }
            let vec = get_data(string.clone());
            let shape = get_shape(path.path());
            let activation = get_act(path.path());
            nn.add(ModuleType::from_vector(activation, Matrix::from_vec(shape.0, shape.1, vec)));
            string = String::new();


      //      println!("Vec {:?}", vec);
        }
        return nn;
    }
}
fn get_shape(path: std::path::PathBuf) -> (usize, usize) {
    let file = path.file_name().unwrap();
    let mut state = 0;
    let mut rows: String = String::new();
    let mut cols: String = String::new();
    for x in file.to_str().unwrap().chars() {
        if x == '(' {
            if state == 0 {
                state = 1;
            }
            
            continue;
        }
        if x == ',' {
            state = 2;
            continue;
        }
        if x == ')' {
            state = 0;
            continue;
        }  
        if state == 1 {
            rows.push(x);
        }
        if state == 2 {
            cols.push(x);
        }
    }
 //   println!("{}", rows);
    rows = rows.chars().filter(|c| !c.is_whitespace()).collect();
    cols = cols.chars().filter(|c| !c.is_whitespace()).collect();
    let rows: usize = rows.parse().unwrap();
    let cols: usize = cols.parse().unwrap();
    return (rows, cols);
    
}
fn get_act(path: std::path::PathBuf) -> ActivationType {
    let mut activation = ActivationType::None;
    let file = path.file_name().unwrap();
    let mut state = 0;
    let mut vec: Vec<String> = Vec::new();
    for x in file.to_str().unwrap().chars() {
        if x == '#' {
            if state == 0 {
                state = 1;
            } else {
                state = 0;
            }
            continue;
        }
        if state == 1 {
            vec.push(x.to_string());
        }
    //    println!("{:?}", x);
    }
    if vec.concat() == "Sigmoid" {
        activation = ActivationType::Sigmoid;
    }
    if vec.concat() == "Relu" {
        activation = ActivationType::Relu;
    }
    if vec.concat() == "None" {
        activation = ActivationType::None;
    }
    activation


}

fn get_data(data: String) -> Vec<f32> {
    let mut token = String::new();
    let mut real_vec: Vec<f32> = Vec::new();
    let mut string_putting = String::new();
    let mut state = 0;

    for c in data.chars() {
        if state == 0 {
            token.push(c);
        }
        if state == 1 {
            if c != ',' && c != ']' {
                string_putting.push(c);
            } else {
                state = 0;
                string_putting = string_putting.chars().filter(|c| !c.is_whitespace()).collect();
                real_vec.push(string_putting.parse().unwrap());
                string_putting = String::new();
            }
        }
        if token == " " || token == "[" {
            state = 1;
        }
    }  
    real_vec
}
   