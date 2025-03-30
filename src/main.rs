use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Please Input your Guess: ");

    let mut guess = String::new();
    let mut exit: String = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to Read Line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Number too small!"),
        Ordering::Equal => println!("Correct!"),
        Ordering::Greater => println!("Number too big!") 
    }

        println!("Random num was: {}",secret_number);
        println!("Press any key to exit");
        io::stdin().read_line(&mut exit).expect("Failed to Read Line");
        println!("You exited with {}",exit);
}