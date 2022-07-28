use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(unused)]

struct Node<T> {
    data: T,
    parent: RefCell<Weak<Node<T>>>,
    children: Vec<Rc<Node<T>>>,
}

fn main() {
    let leaf = Rc::from(
	Node::<i32> {
	    data: 15,
	    parent: RefCell::new(Weak::new()),
	    children: vec![],
	}
    );

    let root = Rc::from(
	Node::<i32> {
	    data: 0,
	    parent: RefCell::new(Weak::new()),
	    children: vec![Rc::clone(&leaf)],
	}
    );

    *leaf.parent.borrow_mut() = Weak::from(Rc::downgrade(&root));
}
