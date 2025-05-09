use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

// https://gist.github.com/jonhoo/935060885d0d832d463fda3c89e8259d

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

struct Sender<T> {
    shared: Arc<Shared<T>>
}

impl <T> Clone for Sender<T> {
    fn clone(&self) ->Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.sender_count += 1;
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.sender_count -= 1;
        let was_last = inner.sender_count == 0;
        drop(inner);
        if was_last { self.shared.available.notify_one() };
    }
}

// By default generated
// impl<T: Clone> Clone for Sender<T> {
//     fn clone(&self) {
//         // ...
//     }
// }

impl<T> Sender<T> {
    fn send(&mut self, val: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(val);
        self.shared.available.notify_one();
    }
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn receive(&mut self) -> Option<T> {
        if let Some(val) = self.buffer.pop_front() {
            return Some(val);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front()  {
                Some(val) => {
                    std::mem::swap(&mut self.buffer, &mut inner.queue);
                    return Some(val);
                }
                None if inner.sender_count == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    sender_count: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

fn channel<T>() -> (Sender<T>, Receiver<T>)  {
    let inner = Inner {
        queue: VecDeque::default(),
        sender_count: 1
    };

    let shared = Arc::new(
        Shared {
            inner: Mutex::new(inner),
            available: Condvar::new(),
        }
    );

    (
        Sender {
            shared: shared.clone() 
        },
        Receiver { 
            shared: shared.clone(),
            buffer: VecDeque::default()
        }
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(2);
        assert_eq!(rx.receive(), Some(2));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.receive(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}