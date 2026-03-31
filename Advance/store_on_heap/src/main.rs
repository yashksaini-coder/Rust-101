enum List {
    Cons(i32, Box<List>),  // A node with a value and pointer to the rest
    Nil,                    // The empty list (terminator)
}

fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(value, tail) => value + list_sum(tail),
        List::Nil => 0,
    }
}

fn main() {
    // Create a list: 1 -> 2 -> 3 -> Nil
    let my_list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    // Calculate and print the sum
    let sum = list_sum(&my_list);
    println!("Sum of list: {}", sum);  // Output: Sum of list: 6
}