use std::{fmt::Display, ops::Deref};

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_and_deref() {
        let x = 5;
        let y = MyBox::new(5);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
