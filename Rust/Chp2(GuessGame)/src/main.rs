extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guest the Number!");

    let mut secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your Guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your Guess: {}", guess);

        match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => {
            println!("You win!");
            
            println!("Do you want to play again press Y | N");
            let mut check = String::new();
            io::stdin().read_line(&mut check).expect("Failed to read line");
            if check=="y\n"{
                secret_number = rand::thread_rng().gen_range(1,101);
                 println!("The secret number is: {}", secret_number);
                continue;
            }else{
                break;
            }

            }
        }

    }
    
}
