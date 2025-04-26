use std::cell:UnsafeCell;

pub struct RefCell<T> {
    data: UnsafeCell<T>,
    reference: isize,
}

implt<T> RefCell {
    pub fn new(value: T) {
        Self {
            data: UnsafeCell::new(T),
            reference: 0,
        }
    }
}