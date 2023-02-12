use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::List::{Cons, Nil};

    #[test]
    fn test_ref_counting() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        assert_eq!(1, Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        assert_eq!(2, Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            let c = Cons(4, Rc::clone(&a));
            assert_eq!(4, Rc::strong_count(&a));
        }
        let c = Cons(4, Rc::clone(&a));
        assert_eq!(3, Rc::strong_count(&a));
    }
}
