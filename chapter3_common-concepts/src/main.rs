// use std::io;

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

    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");

    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Index entered was not a number");

    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    // Functions
    // println!("Hello, World");

    // another_function();
    // another_function(5);
    // print_labeled_measurement(5, 'h');

    // Statements and Expression
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {y}");
    // let x = five();

    // println!("The value of x is : {x}");

    // let x = plus_one(5);

    // println!("The value of x is: {x}");

    // Control Flow
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let number = 3;
    // if number != 0 {
    //     println!("number was three");
    // } else if number > 3 {
    //     println!(" number is bigger than three");
    // }

    // let number = 5;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("The value is: {element}");
    // }
    for number in (1..10).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
// fn another_function() {
//     println!("Another function.");
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn five() -> i32 {
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
