use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

// use std::io::stdin;

fn main() {
    println!("Hello World");
    // datatypes will be guessed if not set, once set are immutable
    let _num = 10;
    // can set mutable variable
    let mut age: i32 = 40;
    // max min data types numbers
    println!("Max i8 {}", i8::MAX);
    println!("Max i8 {}", i8::MIN);
    println!("Max i16 {}", i16::MAX);
    println!("Max i16 {}", i16::MIN);
    println!("Max i32 {}", i32::MAX);
    println!("Max i32 {}", i32::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Max i64 {}", i64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Max isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Max usize {}", usize::MIN);
    println!("Max f32 {}", f32::MAX);
    println!("Max f32 {}", f32::MIN);
    println!("Max f64 {}", f64::MAX);
    println!("Max f64 {}", f64::MIN);
    println!("Max u8 {}", u8::MAX);
    println!("Max u8 {}", u8::MIN);
    println!("Max u16 {}", u16::MAX);
    println!("Max u16 {}", u16::MIN);
    println!("Max u32 {}", u32::MAX);
    println!("Max u32 {}", u32::MIN);
    println!("Max u64 {}", u64::MAX);
    println!("Max u64 {}", u64::MIN);

    // booleans
    let is_it_true: bool = true;
    // store individual characters
    let let_x: char = 'x';
    // variables outside of strings
    println!("I am {} years old", age);

    // set multiple values begin with letters or numbers
    let (f_name, l_name) = ("Bobby", "Keel");
    // load different indexes 
    println!("It is {0} that {1} is {0}", is_it_true, let_x);

    println!("{:.1}", 1.234);
    // hexadecimal conversions
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    //define white space/ define named arguments and white space
    println!("{ten:>ws$}", ten=15, ws=5);

    // prefill with zeros instead of whitespace
    println!("{ten:>0ws$}", ten=15, ws=5);

    // math functions
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    // mutable with chaining functions
    let mut neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 27 = {}", 27f64.cbrt());
    println!("Round 1.45 = {}", 1.45f64.round());
    println!("Floor 1.45 = {}", 1.45f64.floor());
    println!("ceiling 1.45 = {}", 1.45f64.ceil());
    println!("abs(-4) = {}", neg_4.abs());
    println!("abs(-4) = {}", neg_4.abs());
    println!("abs(-4) = {}", neg_4.abs());
    println!("abs(-4) = {}", neg_4.abs());
    println!("abs(-4) = {}", neg_4.abs());


}