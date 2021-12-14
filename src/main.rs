use core::num;
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

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
    println!("5 / 4 = {}", 5f32 / 4f32);
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
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to Radians = {}", 90f64.to_radians());
    println!("PI to degrees = {}", 3.14f64.to_degrees());
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));
    println!("Sin 3.14 = {}", 3.14f64.sin());

    // conditional operators != > < == >= <= && || !
    let age_old = 6;

    if(age_old == 5){
        println!("Go to Kindergarten");
    } else if(age_old > 5) && (age_old <= 18){
        println!("Go to grade {}", (age_old - 5))
    } else if(age_old <= 25) && (age_old > 18){
        println!("Go to college");
    } else {
        println!("Do what you want");
    }

    println!("!true = {}", !true);
    println!("true || false = {}", true || false);
    println!("true != false : {}", (true != false));
    // make a turnary operator
    let can_vote = if (age_old >= 18) {true} else {false};
    println!("Can vote: {}", can_vote);

    // looping
    let mut x = 1;

    loop {
        if((x % 2) == 0){
            println!("{}", x);
            x += 1;

            continue;
        }
        if(x > 10){
            break;
        }
        x += 1;
        continue;
    }

    //while loop
    let mut y = 1;

    while y <= 10 {
        println!("{}", y);
        y += 1;
    }

    // for loop
    for z in 1..10 {
        println!("FOR: {}", z);
    }

    // strings
    let rand_string = "I am a random string";

    println!("Length : {}", rand_string.len());

    let (first, second) = rand_string.split_at(6);
    println!("First : {}\nSecond : {}", first, second);

    // iterator for string
    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();

    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

    // returning individual words in a string
    let mut iter = rand_string.split_whitespace();

    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_word = iter.next();
    }

    // returning individual lines in a string
    let rand_string2 = "I am a random string\nThere are other strings like it\nThis string is the best";

    let mut lines = rand_string2.lines();
    let mut indiv_line = lines.next();
    loop {
        match indiv_line {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_line = lines.next();
    }

    println!("Find Best : {}", rand_string2.contains("best"));

    // number guesing game, name loop
    // 'outer: loop{
    //     let number: i32 = 10;
    //     println!("Pick a number");

    //     loop {
    //         let mut line = String::new();

    //         let input = stdin().read_line(&mut line);

    //         let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

    //         match guess {
    //             None => println!("Enter a number"),
    //             Some(n) if n == number => {
    //                 println!("You guessed it");
    //                 // break outer loop
    //                 break 'outer;
    //             }
    //             Some(n) if n < number => println!("Too Low"),
    //             Some(n) if n > number => println!("Too High"),
    //             Some(_) => println!("Error")
    //         }
    //     }
    // }

    // lists check out more on arrays in rust docs
    let rand_array = [1,2,3];

    println!("{}", rand_array[0]);

    println!("{}", rand_array.len());

    println!("Second 2 : {:?}", &rand_array[1..3]);

    //vectors, unlike arrays vectors can grow in size
    let mut vect1 = vec![1,2,3,4,5];
    println!("Item 2 : {}", vect1[1]);

    for i in &vect1 {
        println!("Vect: {}", i);
    }

    vect1.push(6);

    vect1.pop();

    // tuples, chek out more on tuples in rust docs
    let rand_tuple = ("Bobby", 40);

    let rand_tuple_2: (&str, i8) = ("Bobby", 40);

    println!("Name : {}", rand_tuple_2.0);

    say_hello("Bobby");

    println!("5 + 4 = {}", get_sum(5, 4));

    let sum = get_sum;
    println!("6 + 4 = {}", sum(6,4));

    // closures, can be passed into a function
    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7,8));

    // can access variables outside of closures, but with functions cannot
    let num_ten = 10;

    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));

    // ownership, pointers as references
    
    // none primitive value, cannot access moved nonprim value
    // let vect1 = vec![1,2,3];
    // let vect2 = vect1;
    // println!("vect1[0] : {}", vect1[0]);

    // primitive values, can access moved prim val
    let prim_val = 1;
    let prim_val2 = prim_val;
    println!("prim_val : {}", prim_val);


    let vect2 = vec![1,2,3];
    println!("Sum of Vect: {}", sum_vects(&vect2));
    println!("Vect : {:?}", vect2);

    // create circle
    let mut circle1 = Circle {
        x: 10.0, y: 20.0, radius: 18.0
    };

    println!("X : {}, Y : {}, R: {}", circle1.x, circle1.y, circle1.radius);

    println!("Circle Radius : {}", get_radius(&circle1));

    println!("Circle X : {}", circle1.get_x());

    println!("Circle Area : {}", circle1.area());

    let mut rect1 = Rectangle {
        height: 20.0, width: 10.0
    };

    println!("Rectangle Area : {}", rect1.area());

    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned()};

    get_info(hulk);
    get_info(spiderman);

}

// functions, created outside of main()
fn say_hello(name: &str){
    println!("Hello {}!", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    return sum;
}

// structs
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

// pub makes this fiuntion public then it can be accessed outside the function
impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

// traits
struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

// enumerated types 1 of several values
enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero){
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        },
    }
}
