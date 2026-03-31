use std::ops::Deref;
struct Wrapper<T>(T);

impl <T>Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn double_len(s: &str) -> usize {
    s.len() * 2
}


fn main() {
    let wrapped = Wrapper(String::from("hello"));

    // &Wrapper<String> → &String → &str via deref coercion
    let result = double_len(&wrapped);

    println!("Double length: {}", result);  // Output: 10
}
