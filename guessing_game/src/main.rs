
use std::io;

fn main() {
    println!("game start!");
    println!("Guess a number from 1-100, please enter: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("invaild value!");

    println!("you input number: {}", guess);
}
