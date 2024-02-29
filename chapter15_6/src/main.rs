use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

struct BadNode {
    value: i32,
    children: RefCell<Vec<Rc<BadNode>>>,
}

#[derive(Debug)]
struct GoodNode {
    value: i32,
    parent: RefCell<Weak<GoodNode>>,
    children: RefCell<Vec<Rc<GoodNode>>>,
}

fn main() {
    {
        // Creating a Reference Cycle
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a item = {:?}", a);
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b item = {:?}", b);
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());

        // Creating a reference cycle would be a logic bug
        // use automated tests, code reviews, and other software development practices to minimize
    }
    {
        // Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
        let bad_leaf = Rc::new(BadNode {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let bad_branch = Rc::new(BadNode {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&bad_leaf)]),
        });

        // Adding a Reference from a Child to its Parent
        let good_leaf = Rc::new(GoodNode {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("good_leaf parent = {:?}", good_leaf.parent.borrow().upgrade());

        let branch = Rc::new(GoodNode {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&good_leaf)]),
        });

        *good_leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("good leaf parent = {:?}", good_leaf.parent.borrow().upgrade());
    }
    {
        // Visualizing Changes to strong_count and weak_count
        let leaf = Rc::new(GoodNode {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}", // leaf strong = 1, weak = 0
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(GoodNode {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}", // leaf strong = 1, weak = 0
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}", // leaf strong = 2, weak = 0
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None
        println!(
            "leaf strong = {}, weak = {}", // leaf strong = 1, weak = 0
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
