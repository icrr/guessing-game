use std::io;
use rand::Rng;

enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub fn main() {

    let difficulty = get_difficulty();
    let secret_number = generate_secret_number(&difficulty);
    println!("Enter a number");

    let mut attempts: i32 = 0;

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

    attempts += 1;

    println!("You typed: {}", input);

    if secret_number == input {
        println!("You win! You got it right in {} tries", attempts);
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

fn get_difficulty() -> Difficulty {

    println!("Choose a difficulty level:
        1 - Easy
        2 - Medium
        3 - Hard
    ");

    loop {
        let mut define = String::new();
        io::stdin()
        .read_line(&mut define)
        .expect("Failed to read line");

        match define.trim().parse::<i32>() {
            Ok(1) => return Difficulty::Easy,
            Ok(2) => return Difficulty::Medium,
            Ok(3) => return Difficulty::Hard,
            _ => println!("Invalid input! Please, choose a valid option."),
        }

        for 1 in define {
            attempts < 10;
        }
    }
}


fn generate_secret_number(difficulty: &Difficulty) -> i32 {
    match difficulty {
        Difficulty::Easy => rand::thread_rng().gen_range(1..=10),
        Difficulty::Medium => rand::thread_rng().gen_range(1..=50),
        Difficulty::Hard => rand::thread_rng().gen_range(1..=100),
    }
}

