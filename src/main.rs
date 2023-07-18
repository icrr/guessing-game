use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Enter a number");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please, type a number.");

    println!("You typed: {}", input);

    match input.cmp(&secret_number) {
        Less => println!("Too small!"),
        Greater => println!("Too big!"),
        Equal => println!("You win!"),
    }
}

