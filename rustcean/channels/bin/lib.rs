use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//  - Mutex + Condvar + VecDeque
//  - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify

//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of atomic VecDeque<T>

//  - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//  - Oneshot channels: Any capacity. In practice, only one call to send().

#[derive(Clone)]
struct Sender<T> {
    shared: Arc<Shared<T>>
}


// impl<T: Clone> Clone for Sender<T> {
//     fn clone(&self) {
//         // ...
//     }
// }


impl<T> Sender<T> {
    fn send(&self, val: T) {
        let guard = self.shared.lock().unwrap();
        self.inner.queue.push_back(val);
        drop(guard);
        self.inner.available.notify_one();
    }
}



struct R<T> {
    shared: Arc<Shared<T>>
}

impl<T> R<T> {
    fn receive(&self) -> T {
        let inner = self.shared.lock().unwrap();
        loop {
            let val = queue.unwrap();
            match queue.pop_front()  {
                Some(val) => return val,
                None if
            }
            let val = ;
            self.inner.available.wait(guard).unwrap();
        }
    }
}



#[Clone, Debug, Send, Sync]
struct Shared<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    available: Condvar,
    sender_count: usize,
}

impl<T> Inner<T> {
    fn new() -> Self {
        Inner {
            queue: Arc::new(Mutex::new(VecDeque::new()))
        }
    }
}

struct Channel<T: Clone> {
    fn create(data: T) -> (R<T>, S<T>) {
        (R { val: data.clone() }, S { val: data.clone() })
    }  
}

fn example() {
    let (rx, tx) = Channel::create(Inner::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        example();
    }
}