use std::io;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Enter a number");

    let mut _attempts: i32 = 0;

    loop {

    let mut input = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please, type a number.");
            continue;
        }
    };

    _attempts += 1;

    println!("You typed: {}", input);

    if secret_number == input {
        println!("You win! You got it right in {} tries", _attempts);
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

