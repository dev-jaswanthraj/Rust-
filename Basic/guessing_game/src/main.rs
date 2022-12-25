use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is:{}", secret_number);

    loop{
    
        println!("Please input your guess.");
        let mut guess = String::new();
        // let apple = 5; -> immutable by default
        // let mut apple = 5; -> mutable by using mut keyword
        io::stdin()
            .read_line(&mut guess) // return result variant either Ok or Err
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>  continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
