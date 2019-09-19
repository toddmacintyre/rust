use std::mem;
// use std::convert::TryInto;

fn main() {
    // println!("Hello, world!");
    // let a = true;
    // println!("a = {}, bytes = {}", a, std::mem::size_of_val(&a));
    // let pi = std::f64::consts::PI;
    // println!("{}", std::f64::consts::PI);


    println!("++++++++++", );
    let x = Box::new(2147483647);
    println!("x = {}", std::mem::size_of_val(&*x));
    println!("max i32 = {}", std::u32::MAX);
    let num:u32 = std::i32::MAX as u32 * 2 + 1;
    println!("u32max === i32max * 2 {}:{}, {}", std::u32::MAX, num, std::u32::MAX == num);
}

