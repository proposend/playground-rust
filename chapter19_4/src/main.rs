fn add_one(x: i32) -> i32 {
    x + 1
}

// Function Pointers
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    {
        // Function Pointers
        let answer = do_twice(add_one, 5);
        println!("The answer is {}", answer);

        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        println!("list of strings {:?}", list_of_strings);

        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
        println!("list of strings {:?}", list_of_strings);

        let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
        println!("list of status {:?}", list_of_status);
    }
    {
        // Returning Closures
    }
}

// you’re not allowed to use the function pointer fn as a return type `dyn Fn(i32) -> i32`
// fn returns_closure() -> dyn Fn(i32) -> i32 { // doesn't have a size known at compile-time
//     |x| x + 1
// }
// Rust doesn't know how much space it will need to store the closure. We can use a trait object:
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

