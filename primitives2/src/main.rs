mod tuple;
mod array_slice;

fn main() {
    //add
    println!("1+1= {}", 1i32 + 1);
    //sub
    println!("10-4= {}", 10u32 - 4);
    //bool
    println!("true and true : {}", true && true);
    //bitwise
    println!("1001 OR 0110: {:04b}",0b1001|0b0110);
    //underscores
    println!("100_00: {}",100_00u16);

    //call function from tuple
    tuple::run();
    array_slice::run();
}
