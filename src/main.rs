use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

   // println!("The secret number is: {secret_number}");
    loop {
        
        println!("\x1b[33mPlease input your guess.\x1b[0m");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\x1b[34mToo small\x1b[0m"),
            Ordering::Greater => println!("\x1b[31mToo big\x1b[0m"),
            Ordering::Equal => {
                println!("\x1b[32mYou win\x1b[0m");
                break;
            }
        }
    
        println!("\x1b[37mYou guessed {}\x1b[0m",  guess);

    }

    

}
