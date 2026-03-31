use std::rc::Rc;

fn main() {

    // simple Rc
    let a = Rc::new(2);
    let b = a.clone();
    let c = a.clone();
    
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    let shared = Rc::new("hello".to_string());
    let _clone1 = Rc::clone(&shared);
    let _clone2 = Rc::clone(&shared);
    println!("{}", Rc::strong_count(&shared)); // 3
}