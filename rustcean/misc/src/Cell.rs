use std::cell::UnsafeCell;

struct Cell<T> {
    data: UnsafeCell<T>;
}

impl<T> Cell<T> {

    pub fn new(value: T) -> Self {
        Cell { data: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        unsafe { self.data.get() = value; }
    }

    pub fn get(&self) -> T where T: Copy {
        unsafe { *self.data.get() }
    }
}

#cfg[test]
mod test {
    use super::cell;

    #[ test]
    fn bad() {
        use std::sync::Arc;

        let x = Arc::new(Cell::new(2));
        let x1 = x.clone();

        std::thread::spawn (move || {
            x1.set(1);
        });

        let x2 = x.clone();

        std::thread::spawn (move || {
            x2.set(2);
        });
    }

    #[test]
    fn bad2() {
        let x = Cell::new(vec![1]);
        let val = &x.get()[0];
        x.set(vec![]);
        println!("val {}", val);
    }


    #[ test]
    fn bad() {
        use std::sync::Arc;

        let x = Arc::new(Cell::new(0));
        let x1 = x.clone();

        let jh1 = std::thread::spawn (move || {
            for i in 0..100000 {
                x1.set(x1.get() + 1);
            }
        });

        let x2 = x.clone();

        let jh2 = std::thread::spawn (move || {
            for i in 0..100000 {
              x2.set(x2.get() + 1);
            }
        });
        
        jh1.join();
        jh2.join();

    }

}