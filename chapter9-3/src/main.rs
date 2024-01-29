// use std::net::IpAddr;
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, Got {}", value);
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    // {
    //     let home: IpAddr = "127.0.0.1"
    //         .parse()
    //         .expect("Hardcoded IP Address should be valid");

    //     println!("{:?}", home);
    // }
    let value = Guess::new(10).value;
    println!("value: {}", value);
}
