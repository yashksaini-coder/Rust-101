use std::rc::Rc;

fn count_owners(n: usize) -> usize {
    // Step 1: Create an Rc<String> holding "shared"
    let shared = Rc::new("shared".to_string());

    // Step 2: Clone it n times, storing clones in a Vec
    let mut clones = Vec::new();
    for _ in 0..n {
        clones.push(Rc::clone(&shared));  // Increment reference count
    }

    // Step 3: Return the strong count (original + n clones = n + 1)
    Rc::strong_count(&shared)
}

fn main() {
    // Test with n = 5: should return 6 (original + 5 clones)
    let result = count_owners(5);
    println!("Number of owners: {}", result);  // Output: 6

    // Verify the pattern holds
    assert_eq!(count_owners(0), 1);   // Just the original
    assert_eq!(count_owners(3), 4);   // Original + 3
    assert_eq!(count_owners(10), 11); // Original + 10
}