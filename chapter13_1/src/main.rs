use std::{thread, time::Duration};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    // {
    //     // Closure Type Inference and Annotation
    //     let expensive_closure = |num: u32| -> u32 {
    //         println!("calculating slowly...");
    //         thread::sleep(Duration::from_secs(2));
    //         num
    //     };
    // }
    // {
    //     // Capturing References or Moving Ownership
    //     let list = vec![1, 2, 3];
    //     println!("Before defining closure: {:?}", list);

    //     let only_borrows = || println!("From closure: {:?}", list);

    //     println!("Before calling closure: {:?}", list);
    //     only_borrows();
    //     println!("After calling closure: {:?}", list);
    // }
    // {
    //     let mut list = vec![1, 2, 3];
    //     println!("Before defining closure: {:?}", list);

    //     let mut borrows_mutably = || list.push(7);
    //     // println!("Before calling closure: {:?}", list);
    //     borrows_mutably();
    //     println!("After calling closure: {:?}", list);
    // }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();

        // println!("After defining closure: {:?}", list);
    }
    {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];
        //  |r| r.width는 자신의 환경으로부터 어떤 것도 캡처나 변형, 혹은 이동을 시키지 않으므로, 트레이트 바운드 요건을 충족
        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);

        // let mut sort_operations = vec![];
        // let value = String::from("by key called");

        // list.sort_by_key(|r| {
        // value 값이 처음 호출되고나면, 그 다음 VALUE 호출은 유효하지않게됨.
        //     sort_operations.push(value);
        //     r.width
        // });
        // println!("{:#?}", list);

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            // 한 번 이상 호출 가능
            num_sort_operations += 1;
            r.width
        });
        println!("{:#?}, sorted in {num_sort_operations} operations", list);
    }
    {}
}
