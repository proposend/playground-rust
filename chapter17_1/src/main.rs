use chapter17_1::AveragedCollection;

fn main() {
    let mut a = AveragedCollection::new();

    a.add(5);
    a.add(10);
    a.add(100);
    println!("average {}", a.average());
}
