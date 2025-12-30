use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses:u32 = 0;

    loop {
        println!("Enter the guessed number"); 
        number_of_guesses+=1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number only!! ");
                number_of_guesses-=1;
                continue;
            }
        };

        // println!("Your guess :{guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too less"),
            Ordering::Greater=> println!("too big"),
            Ordering::Equal=> {
                println!("sweet spot");
                break ;
            }
        }
    }

    println!("\n\nNumber of Guesses used : {number_of_guesses}");
    println!("Secret Number was : {secret_number}\n\n");
}

