use crate::garden::vegetables::Asparagus;

pub mod garden;
fn main() {
    let plant = Asparagus {
        width: 30,
        height: 50,
    };

    println!("I'm growing {:?}", plant);
}
