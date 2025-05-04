struct Rc<T> {
    inner: *mut Inner<T>,
}

impl<T> Rc<T> {
    fn new(data: T) -> Self {
        Rc {
            inner: Box::into_raw(Box::new(Inner::new(data))),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe { &mut *self.inner }.count += 1;
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let count = &mut unsafe { &mut *self.inner }.count;
        if *count == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            *count -= 1;
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.data
    }
}

struct Inner<T> {
    count: usize,
    data: T,
}

impl<T> Inner<T> {
    fn new(data: T) -> Self {
        Inner { count: 1, data }
    }
}

fn main() {
    let x = Rc::new(1);
    let y = x.clone();
    std::thread::spawn(move || {
        let x = y;
        drop(x);
    });
}
