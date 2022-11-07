//To obtain user input and then print the result as output
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    loop {
        println!("0 >> IF YOU WANT TO FINISH PRESS '0'");
        //we generate a secret number gen_range(start..=end)
        let secret_number = rand::thread_rng().gen_range(1..=10);

        println!("1 >> Please input a guess");

        //create a variable to store the user input
        //we made the vatiable mutable using 'mut'
        //String::new is a function that returns a new instance of a String
        let mut guess = String::new();

        //stdin function will allow us to handle user input
        io::stdin()
            //the read_line method on the standard input handle to get input from the user
            //we pass the variable as argument to tell it what string to store the user input in
            //The '&' indicates that this argument is a reference
            //read_line returns a Result value
            .read_line(&mut guess)
            //expect is handling a potencial error with 'RESTUL TYPE'
            .expect("ERR >> Failed to read line!");

        //convert the guess to u32 because we can not compare an string with a number
        //trim() will eliminate any whitespace at the beginning and end
        //parse() converts a string to another type (te type is : u32)
        //parse will only work on characters that can logically be converted into numbers
        // let guess: u32 = guess.trim().parse().expect("ERR >> Please type a number!");

        //we see what is the result type and based in the response 
        //do either return num or continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess == 0 {
            break;
        }

        println!("2 >> Your guessed: {guess}");

        //we call a function that compares two variables guess.compare('reference of secret_number')
        //with three outcomes that are possible when you compare two values

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("3 >> Too small!"),
            Ordering::Greater => println!("3 >> Too big!"),
            Ordering::Equal => {
                println!("3 >> U win!");
                break;
            }
        }

        println!("4 >> The secret number is: {}", secret_number);
    }
}
