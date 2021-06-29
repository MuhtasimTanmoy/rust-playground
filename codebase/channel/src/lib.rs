use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl <T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>
}

impl <T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    self.inner.available.wait(queue);
                }
            }
        }
    }
}

pub struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}
 
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar:: new(),
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
