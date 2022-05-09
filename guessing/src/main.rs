use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Calculate guessing number...");
    let secret_number  = rand::thread_rng().gen_range(1..101); 

    println!("Print guessing number, ...!");
    let mut guess = String::new();

    println!("Secret number is {}", secret_number);
    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line!");
    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Equal"),
    }

}
