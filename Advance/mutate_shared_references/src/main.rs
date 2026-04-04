use std::cell::RefCell;

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    // Constructor (not required but helpful)
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    // Mutates through shared &self reference
    fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    // Returns current value
    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

fn count_to(n: i32) -> i32 {
    let counter = Counter::new();
    for _ in 0..n {
        counter.increment();  // Works even with immutable binding!
    }
    counter.get()
}

fn main() {
    let count = count_to(5);
    println!("Final count: {}", count);  // Output: 5

    // Demonstrating shared mutability
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("After 2 increments: {}", counter.get());  // Output: 2
}