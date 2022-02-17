
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random = rand::thread_rng().gen_range(1..101);

    loop {
        println!("");
        println!("game start!");
        println!("Guess a number from 1-100, please enter: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you input number: {}", guess);
        // println!("random number: {}", &random);

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
