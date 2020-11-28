use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1, 100);
    // println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Input your guess");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
