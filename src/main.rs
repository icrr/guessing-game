use std::io;
use rand::Rng;

fn main() {

    loop {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number: {}", secret_number);
    println!("Enter a number");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please, type a number.");

    println!("You typed: {}", input);

    if secret_number == input {
        println!("You win!");
        break;
    } else if secret_number > input {
        println!("The secret number is bigger!");
        continue;
    } else {
        println!("The secret number is smaller!");
        continue;
    }
}
}

