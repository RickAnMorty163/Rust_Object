use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");

    let mut rand_ = rand::thread_rng().gen_range(1, 101);

    let mut count: u32 = 0;

    loop {
        println!("Please guess a number:");

        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is:{}", guess_number);

        count+=1;

        match guess_number.cmp(&mut rand_) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You are winner!");
                break;
            }
        }
    }
    println!("The match count is:{}", count);
}
