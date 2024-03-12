use std::fmt;

// Creating Type Synonyms with Type Aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;


type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<(usize)>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    {
        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }
    {
        // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        let f: Thunk = Box::new(|| println!("hi"));
    }
    {
        // The Never Type that Never Returns
        // bar();
        println!("This line will never be reached!");
    }
    {
        // Dynamically Sized Types and the Sized Trait
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";

        let s1: &str = "Hello there!";
        let s2: &str = "How's it going?";
    }
}

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
fn takes_long_type(f: Thunk) {}

//fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}
// fn returns_long_type() -> Thunk {}


// fn bar() -> ! {
//     // --snip--
//     // panic!("This function will never return!");
// }

// fn generic<T>(t: T) {}
// fn generic<T: Sized>(t: T) {}
// fn generic<T: ?Sized>(t: &T) {}