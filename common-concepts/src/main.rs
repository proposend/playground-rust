use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is : {x}");
    // x = 6;
    // println!("The value of x is : {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // println!("Constants : {THREE_HOURS_IN_SECONDS}")

    // Shadowing
    // let x = 5;
    // println!("The value of x is : {x}");

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!(" The value of x  in the inner scope is : {x}");
    // }

    // println!("The value of x is : {x}");

    // let spaces = "    ";
    // let spaces = spaces.len();

    // println!("The value of space is : {spaces}");

    // mut 으로 선언한 변수는 최초 할당된 타입을 변경할수없음
    // let mut spaces = "    ";
    // let spaces = spaces.len();
    // println!("The value of space is : {spaces}");

    // Data Types
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("guess: {guess}");

    // Scalar Types
    // Integer, Floating-point numbers, Boolean, Characters
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // let remainder = 43 % 5;

    // println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    // // The Boolean Type
    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';

    // println!("{t},{f},{c},{z},{heart_eyed_cat}");

    // Compound Types
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of xyz is: {x},{y},{z}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // println!("{five_hundred}, {six_point_four}, {one}");

    // The Array Type
    // let a = [1,2,3,4,5];

    // let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // let a= [3;5]; // let a = [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");
    
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
