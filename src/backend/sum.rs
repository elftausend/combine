use super::Number;


pub fn th_s<T: 'static>(data2: Vec<T>) -> std::thread::JoinHandle<T> where T: Send + Number + std::fmt::Debug {
    let builder2 = std::thread::Builder::new();

    builder2.spawn(move || {
        let mut x = T::number(0.);
        for d in data2 {
            //println!("d: {:?}", d);
            x = T::add(d, x);
        }
        x
    }).unwrap()
}