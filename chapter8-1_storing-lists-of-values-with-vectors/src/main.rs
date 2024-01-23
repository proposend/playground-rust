#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    // Creating a New Vector
    // {
    //     let v: Vec<i32> = Vec::new();
    //     let v = vec![1, 2, 3];
    // }
    // Updating a Vector
    // {
    //     let mut v = Vec::new();

    //     v.push(5);
    //     v.push(6);
    //     v.push(7);
    //     v.push(8);

    //     print!("v :{:?}", v);
    // }
    // Reading Elements of Vectors
    // {
    //     let v = vec![1, 2, 3, 4, 5];
    //     let third: &i32 = &v[2];
    //     println!("The third element is {third}");

    //     let third: Option<&i32> = v.get(2);
    //     match third {
    //         Some(third) => println!("The third element is {third}"),
    //         None => println!("There is no third element"),
    //     }
    // }
    // {
    //     let v = vec![1, 2, 3, 4, 5];

    //     let does_not_exist = &v[100]; // panicking
    //     let does_not_exist = v.get(100);
    // }
    // {
    //     let mut v = vec![1, 2, 3, 4, 5];
    //     let first = &v[0]; // immutable borrow occurs here
    //     v.push(6); // mutable borrow occurs here
    //     println!("The fist element is: {first}"); // immutable borrow later used here
    // }
    // Iterating over the Values in a Vector
    // {
    //     let v = vec![100, 32, 57];
    //     for i in &v {
    //         println!("{i}");
    //     }

    //     let mut v = vec![100, 32, 57];
    //     for i in &mut v {
    //         *i += 50;
    //         println!("{i}");
    //     }
    // }
    // Using and Enum to Store Multiple Types
    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row {
            println!("{:?}", i);
        }
    }
    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];
    }
}
