fn main() {
    // Integer types
    let x: i32 = 42;
    let y: u64 = 100;
    
    // Floating point
    let pi: f64 = 3.14;
    let e: f32 = 2.71;
    
    // String and char
    let name: &str = "Rust";
    let character: char = 'R';
    
    // Boolean
    let value: bool = true;
    
    // Array
    let numbers: [i32; 3] = [1, 2, 3];
    
    // Tuple
    let tuple: (i32, &str, bool) = (42, "hello", true);
    
    println!("Integer (i32): {}", x);
    println!("Unsigned Integer (u64): {}", y);
    println!("Float (f64): {}", pi);
    println!("Float (f32): {}", e);
    println!("String: {}", name);
    println!("Char: {}", character);
    println!("Boolean: {}", value);
    println!("Array: {:?}", numbers);
    println!("Tuple: {:?}", tuple);
}
