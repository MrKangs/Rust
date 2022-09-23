use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_num);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess // u32 is unsigned int 32
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_) => continue
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num){
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Less => println!("{}", "Too Small".red())
        }
    }
}
