use std::cell::RefCell;

fn main() {
    let mut a = String::from("hello");
    let _b = a;
    a.push_str(", world!");

    println!("{}", a);
}