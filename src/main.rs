extern crate futures;

use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Arc,RwLock};
use futures::{Future, Poll, Async, task};
use futures::task::Task;

struct DequeueFuture<'a, T: 'a + Clone> {
  future_queue: &'a mut FutureQueue<T>
}

impl<'a, T: Clone> DequeueFuture<'a, T> {
  fn new(future_queue: &'a mut FutureQueue<T>) -> DequeueFuture<'a, T> {
    DequeueFuture {
      future_queue: future_queue
    }
  }
}

impl<'a, T: Clone> Future for DequeueFuture<'a, T> {
  type Item = T;
  type Error = usize;

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    println!("POLL");
    match self.future_queue.next() {
      Some(t) => Ok(Async::Ready(t)),
      None => {
        self.future_queue.register(task::park());
        Ok(Async::NotReady)
      }
    }
  }
}

#[derive(Clone)]
struct FutureQueue<T: Clone> {
  queue: VecDeque<T>,
  tasks: VecDeque<Task>
}

impl<T: Clone> FutureQueue<T> {
  fn new() -> FutureQueue<T> {
    FutureQueue {
      queue: VecDeque::new(),
      tasks: VecDeque::new()
    }
  }

  fn enqueue(&mut self, t: T) {
    println!("ENQUEUE");
    self.queue.push_front(t);
    println!("{:?}", self.tasks.len());
    if let Some(task) = self.tasks.pop_back() {
      task.unpark();
    }
  }

  fn register(&mut self, task: Task) {
    self.tasks.push_front(task);
  }

  fn next(&mut self) -> Option<T> {
    self.queue.pop_back()
  }

  fn dequeue(&mut self) -> DequeueFuture<T> {
    DequeueFuture::new(self)
  }
}

fn main() {
  let future_queue = Arc::new(RwLock::new(FutureQueue::new()));

  let cloned = future_queue.clone();
  thread::spawn(move|| {
    loop {
      thread::sleep(Duration::new(5, 0));
      println!("enqueue");
      cloned.write().unwrap().enqueue(1);
    }
  });

  let cloned2 = future_queue.clone();
  thread::spawn(move || {
    let future = {  cloned2.write().unwrap().dequeue() };
    println!("{:?}", future.wait());
  }).join().unwrap();
}
