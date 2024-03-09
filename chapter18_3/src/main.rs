#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct SecondPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum HelloMessage {
    Hello { id: i32 },
}

enum NewMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    {
        // Matching Literals
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        // Matching Named Variables
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
    {
        // Multiple Patterns
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        // Matching Ranges of Values with ..=
        let x = 5;
        match x {
            // match 패턴에서 1..5 와 같은 반열린범위 패턴은 사용불가능. 1..=5 와 같은 폐쇠범위 패턴 사용
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    {
        // Destructuring to Break Apart Values
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }
    {
        // Destructuring Enums
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x}, and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}");
            }
        }
    }
    {
        // Destructuring Nested Structs and Enums
        let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
            _ => (),
        }
    }
    {
        // Destructuring Structs and Tuples
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

        println!("feet :{feet}, inches: {inches}, x:{x}, y:{y}");
    }
    {
        // Ignoring an Entire Value with _
        foo(3, 4);
    }
    {
        // Ignoring Parts of a Value with a Nested _
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, thrid, _, fifth) => {
                println!("Some numbers: {first}, {thrid}, {fifth}");
            }
        }
    }
    {
        // Ignoring an Unused Variable by Starting Its Name with _
        let _x = 5;
        let y = 10; // ^ help: if this is intentional, prefix it with an underscore: `_y`

        let s = Some(String::from("Hello"));

        if let Some(_s) = s { // -- value partially moved here
            println!("found a string");
        }

        // println!("{:?}", s); // ^ value borrowed here after partial move

        let s = Some(String::from("Hello"));

        if let Some(_) = s { //
            println!("found a string");
        }
        println!("{:?}", s);
    }
    {
        // Ignoring Remaining Parts of a Value with ..
        let origin = SecondPoint { x: 0, y: 0, z: 0 };

        match origin {
            SecondPoint { x, .. } => println!("x is {}", x)
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        // match numbers {
        //     (.., second, ..) => { // ..  can only be used once per tuple pattern
        //         println!("Some numbers: {second}");
        //     }
        // }
    }
    {
        // Extra Conditionals with Match Guards
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {y}", x);

        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no")
        }
    }
    {
        // @ Bindings
        let msg = HelloMessage::Hello { id: 4 };

        match msg {
            HelloMessage::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in rage : {}", id_variable),
            HelloMessage::Hello {
                id: 10..=12
            } => {
                println!("Found an id in another range");
            }
            HelloMessage::Hello { id } => println!("Found some other id: {}", id)
        }

        // Using @ lets us test a value and save it in a variable within one pattern.
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
