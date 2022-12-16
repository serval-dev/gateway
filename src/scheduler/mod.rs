use std::{collections::VecDeque};


use self::tasks::Task;

mod tasks;

pub struct Scheduler<'a> {
    pub queue: VecDeque<Task<'a>>
}

impl<'a> Scheduler<'a> {
    pub fn enqueue(&self, task: Task) -> &Self {
        // Push it back to the front of the queue.
        self.queue.push_back(task);
        
        if (self.queue.len() != 0) {
            self.run();
        }

        self
    }

    pub fn dequeue(&self) -> Option<Task> {
        self.queue.pop_front()
    }

    pub async fn run(&'a self) {
        // Set an interval for a second.
        let mut task_timer = tokio::time::interval(chrono::Duration::seconds(1).to_std().unwrap());
        loop {
            if (self.queue.len() == 0) {
                // We've reached the end of the queue.
                // Wait a second before trying again.
                task_timer.tick().await;
            }

            // Dequeue a task.
            let mut queue_clone = self.queue.clone();
            let task = queue_clone.pop_front();

            self.queue = queue_clone;

            if (task.is_some()) {
                let unwrapped_task = task.unwrap();

                if (unwrapped_task.block == true) {
                    // Run the function as a blocking call.
                    tokio::task::spawn_blocking(|| unwrapped_task.run());
                } else {
                    // Spawn the function on a seperate thread.
                    tokio::spawn(async {
                        unwrapped_task.run();
                    });
                }
                
            }

        }
    }

    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new()
        }
    }
}