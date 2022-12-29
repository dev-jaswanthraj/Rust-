use std::io;

fn main() {
    println!("Please enter a number");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let n: i32 = input.trim().parse().expect("Error");

    let result = fib(n);
    println!("{}", result)
}

fn fib(n :i32) -> i32{
    if n == 0 {
        return 0
    }
    if n == 1 || n == 2 {
        return 1
    }
    fib(n-1) + fib(n-2)
}
