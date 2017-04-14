extern crate futures;

use std::thread;
use std::collections::VecDeque;
use std::sync::{Arc,RwLock};
use futures::{Future, Poll, Async};

#[derive(Clone, Debug)]
struct FutureQueue<T> {
  queue: Arc<RwLock<VecDeque<T>>>
}

struct DequeueFuture<T> {
  result: Option<T>
}

impl<T> DequeueFuture<T> {
  fn new() -> DequeueFuture<T> {
    DequeueFuture { result: None }
  }
}

impl<T> Future for DequeueFuture<T> {
  type Item = T;
  type Error = usize;

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    // TODO
    println!("POLL");
    Ok(Async::NotReady)
  }
}

impl<T> FutureQueue<T> {
  fn new() -> FutureQueue<T> {
    FutureQueue { queue: Arc::new(RwLock::new(VecDeque::new())) }
  }

  fn enqueue(&mut self, t: T) {
    let mut lock = self.queue.write().unwrap();
    lock.push_front(t);
  }

  // TODO return a future
  fn dequeue(&mut self) -> DequeueFuture<T> {
    DequeueFuture::new()
  }
}

fn main() {
  let mut future_queue = FutureQueue::new();
  future_queue.enqueue(1);

  let mut cloned = future_queue.clone();
  thread::spawn(move || {
    println!("{:?}", cloned.dequeue().wait());
  }).join().unwrap();
}
