
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

#[derive(Clone)]
struct Sender<T> {
    shared: Arc<Shared<T>>
}

#[derive(Clone, Debug)]
struct Shared<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    sender_count: usize
}

fn main() {}