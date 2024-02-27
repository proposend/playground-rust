use std::cell::RefCell;
use std::rc::Rc;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // Rc<T>, Box<T>, RefCell<T>
    // Enforcing Borrowing Rules at Runtime with RefCell<T>
    {
        // Interior Mutability: A Mutable Borrow to an Immutable Value
        let mut x = 5;
        // x is immutable
        let y = &mut x;
    }
    {
        // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}
