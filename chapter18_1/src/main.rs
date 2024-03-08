fn main() {
    {
        // match Arms
        // match VALUE {
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        // }
        // let five = Some(5);
        // let six = plus_one(five);
        // let none = plus_one(None);
        //
        // println!("five:{:?}, six:{:?}, none:{:?}", five, six, none);
    }
    {
        // Conditional if let Expressions
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    {
        // while let Conditional Loops
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    {
        // for Loops
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    {
        // let Statements
        let x = 5;
        // let PATTERN = EXPRESSION;

        let (x, y, z) = (1, 2, 3);
    }
    {
        // Function Parameters
        let point = (3, 5);
        print_coordinates(&point);
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}