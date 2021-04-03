use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

pub struct Inner<T> {
    queue: Mutex<Vec<T>>,
}
 
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default()
    };
    let inner = Arc:: new(inner);
    (
        Sender{
            inner: inner.clone()
        },
        Receiver{
            inner: inner.clone()
        },
    )
}  


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
