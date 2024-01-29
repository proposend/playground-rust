// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // {
    //     // In Function Definitions
    //     let number_list = vec![34, 50, 25, 100, 65];
    //     let result = largest_i32(&number_list);
    //     println!("The largest number is {}", result);

    //     let char_list = vec!['y', 'm', 'a', 'q'];
    //     let result = largest_char(&char_list);
    //     println!("The largest number is {}", result);
    // }
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    {
        // in Struct Definitions
        let integer = Point { x: 5, y: 10 };
        let float = Point {
            x: 1.0f32,
            y: 4.0f32,
        };
        // let wont_work = Point { x: 5, y: 4.0 };
        let will_work = Point2 { x: 5.0, y: 3 };

        let both_integer = Point2 { x: 5, y: 10 };
        let both_float = Point2 { x: 1.0, y: 4.0 };
        let integer_and_float = Point2 { x: 5, y: 4.0 };

        // In Method Definitions

        println!("Integer.x = {}", integer.x());
        println!("distance from orogin: {}", float.distance_from_origin())
    }
    {
        let p1 = Point3 { x: 5, y: 10.4 };
        let p2 = Point3 { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
