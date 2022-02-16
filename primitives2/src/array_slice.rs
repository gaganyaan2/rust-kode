//use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn run() {
    println!("---------print from array_slice.rs----------");
    //fixed array
    let xs: [i32;4] = [ 1, 2, 3, 4 ];
    println!("print first element of array: {}", xs[0]);

    //init with same
    let xy: [i16;100] = [ 115; 100 ];
    println!("print xy : {}",xy[10]);

    //array length
    println!("array length of xy: {}",xy.len());

    analyze_slice(&xs);
}