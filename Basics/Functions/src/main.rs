fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn factorial(n: i32) -> i32 {
    let mut sum = 1;

    for i in 1..=n {
        sum *= i;
    }

    sum
}

fn sum_of_n(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}

fn main() {
    let a = 4;
    let b = 5;

    let res = multiply(a, b);
    println!("The results is {}", res);

    println!("THe factorial of {} is {}", b, factorial(b));

    println!("The sum of {} is {}", b, sum_of_n(b));
}
