use std::io;

fn main() {
    println!("Hello, world!");
    println!("How are you?");

    //add(5, 10);

    println!("The sum is {}", add_input());
}

fn add(x: i32, y: i32){
    let mut sum: i32 = 0;

    sum = x + y;

    println!("The sum of {} and {} is {}", x, y, sum);
}

fn add_input() -> i32 {
    let sum: i32 ;

    println!("Enter first number: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read line");

    println!("Enter second number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Failed to read line");

    let mut x: i32 = first.trim().parse().expect("Please type a number!");
    let mut y:i32 = second.trim().parse().expect("Please type a number!");

    sum = x + y;

    return sum;
}
