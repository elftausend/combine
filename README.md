# Combine

This is a simplistic neural network framework written in Rust. 
The implementation of generics is probably not good and needs some polishing.
The same counts for the whole code but it works for small things.

## Example

My project [GateBuilder] is a whole dedicated example.

[GateBuilder]: https://github.com/elftausend/GateBuilder/blob/main/src/training/train.rs
This example simulates the XOR logic gate.

```rust
use combine::{Matrix, Network, ModuleType, WeightsStore, ActivationType::Sigmoid, Optimizer};

//input values
let xf = vec![0., 0.,
              1., 0.,
              0., 1.,
              1., 1.,];

//corresponding XOR layout/values, if you input the values from 'xf'
let yf = vec![0.,
              1.,
              1.,
              0.];

let x: Matrix<f32> = Matrix::from_vector(4, 2, xf);
//y is the desired output, which the neural network aims to achieve, if you input x
let y: Matrix<f32> = Matrix::from_vector(4, 1, yf); 

let mut nn = Network::new();
nn.aiming(y.clone());
nn.learning_rate(7.);

//.add adds a layer to the neural network
nn.add(structual::ModuleType::new(Sigmoid, 2, 4));
nn.add(structual::ModuleType::new(Sigmoid, 4, 1));

//trains the neural network (50.000 Epochs)
for x in 0..50000 {
    nn.backwards(x.clone());
}
//to safe the current neural network weights
WeightsStore::new().save("./network/");
//this saved weights can be loaded with this line:
WeightsStore::new().construct("./network/");
//there is a multi-threaded method too:
WeightsStore::new().construct2("./network/");
```