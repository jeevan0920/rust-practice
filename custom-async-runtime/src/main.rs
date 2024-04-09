use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// A simple future that completes after being polled `n` times.
struct MyFuture {
    // Number of polls remaining before completion
    polls_remaining: u32,
}

impl Future for MyFuture {
    type Output = String; // The output of the future is a simple String

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polls_remaining == 0 {
            Poll::Ready("Future completed".to_string())
        } else {
            self.polls_remaining -= 1;
            Poll::Pending
        }
    }
}

struct Executor {
    tasks: VecDeque<Box<dyn Future<Output = String> + Unpin>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            tasks: VecDeque::new(),
        }
    }

    // Add a future to the list of tasks to be executed
    fn spawn(&mut self, future: impl Future<Output = String> + 'static + Unpin) {
        self.tasks.push_back(Box::new(future));
    }

    // Run the executor, driving all futures to completion
    fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            // Create a dummy waker. In a real scenario, this would wake up the task for polling.
            let waker = futures::task::noop_waker_ref();
            let mut cx = Context::from_waker(waker);

            // SAFETY: Here, we're converting the `Box<dyn Future>` into `Pin<Box<dyn Future>>`
            // which is required for the `poll` method. This is safe because we're not moving
            // the future after pinning it, and `Box` itself guarantees that the data it owns
            // won't be moved in memory.
            let mut future = Pin::new(task.as_mut());

            // Poll the pinned future
            match future.poll(&mut cx) {
                Poll::Ready(result) => {
                    println!("Task completed with result: {}", result);
                }
                Poll::Pending => {
                    // If the task is not ready, put it back in the queue for another round.
                    self.tasks.push_back(task);
                }
            }
        }
    }
}

fn main() {
    let mut executor = Executor::new();

    // Create an instance of MyFuture that requires two polls to complete
    let my_future = MyFuture { polls_remaining: 2 };

    // Spawn the future onto the executor
    executor.spawn(my_future);

    // Run the executor to completion
    executor.run();
}
