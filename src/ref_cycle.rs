use std::cell::RefCell;
use std::rc::Rc;

use List::*;

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, RefCell<Rc<List>>),
}

fn main() {
    let l1 = Rc::from(Cons(1, RefCell::from(Rc::from(Nil))));
    let l2 = Rc::from(Cons(2, RefCell::from(Rc::clone(&l1))));

    println!("{:?}", l2);

    let l10 = Rc::from(Cons(10, RefCell::from(Rc::from(Nil))));
    let l20 = Rc::from(Cons(20, RefCell::from(Rc::clone(&l10))));

    println!("{:?}", l20);

    if let Cons(_, next) = l1.as_ref() {
	*next.borrow_mut() = Rc::clone(&l20);
    }

    if let Cons(_, next) = l10.as_ref() {
	*next.borrow_mut() = Rc::clone(&l2);
    }

    println!("{} {} {} {}", Rc::strong_count(&l1), Rc::strong_count(&l2), Rc::strong_count(&l10), Rc::strong_count(&l20));

    println!("{:?}", l2);
    
}
