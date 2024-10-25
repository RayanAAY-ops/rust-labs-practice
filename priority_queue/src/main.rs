use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Job {
    name: String,
    priority: usize,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first
        self.priority.cmp(&other.priority) // This is direct comparison
    }
}

// Implementing PartialOrd trait
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implementing PartialEq trait for equality comparison
impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.name == other.name
    }
}

// Implementing Eq trait
impl Eq for Job {}

fn main() {
    let mut job_queue = BinaryHeap::new();

    // Adding jobs with different priorities
    job_queue.push(Job {
        name: String::from("Task 1"),
        priority: 2,
    });
    job_queue.push(Job {
        name: String::from("Task 2"),
        priority: 5,
    });
    job_queue.push(Job {
        name: String::from("Task 3"),
        priority: 1,
    });

    // Processing jobs based on priority
    while let Some(job) = job_queue.pop() {
        println!("Processing job: {:?}", job);
    }
}
