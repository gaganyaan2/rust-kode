fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn run(){
    println!("---------print from tuple.rs----------");
    let long_tuple = (10i16, 2u32, 0.1f32,'a',true,"test");
    println!("long_tuble: {:?}",long_tuple);
    println!("print 2nd value from long_tuple: {}",long_tuple.2);

    let pair = (1, false);
    println!("reverse tuple: {:?}",reverse(pair));

    let matrix = Matrix(1.1,2.2,3.3,4.4);
    println!("{:?}",matrix);

    
}