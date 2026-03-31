fn main() {

    let a  = Box::new(5);
    let b = Box::new(7);
    let c = *a + *b;

    println!("c = {}", c);

}
