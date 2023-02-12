use std::cell::RefCell;
use std::option::Option::None;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_counts_references() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        assert_eq!(1, Rc::strong_count(&leaf));
        assert_eq!(0, Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            assert_eq!(1, Rc::strong_count(&branch));
            assert_eq!(1, Rc::weak_count(&branch));

            assert_eq!(2, Rc::strong_count(&leaf));
            assert_eq!(0, Rc::weak_count(&leaf));
        }

        assert!(leaf.parent.borrow().upgrade().is_none());

        assert_eq!(1, Rc::strong_count(&leaf));
        assert_eq!(0, Rc::weak_count(&leaf));
    }
}
