use rand::Rng;
use std::cmp::Ordering;
use std::io;

// println!("Welcome to guess a number !");

//     let secret_number = rand::thread_rng().gen_range(0..100);

//     println!("Please input a guess");

//     loop {
//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failted to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please type a valid number!");
//                 continue;
//             }
//         };

//         println!("You guessed: {guess}");
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small ! "),
//             Ordering::Greater => println!("Too big !"),
//             Ordering::Equal => {
//                 println!("You win !");
//                 break;
//             }
//         }
//     }
pub fn main() {
    println!("Welcome to guess a number !");
    // Generate a random number

    // let secret_number = rand::thread_rng().gen_range(0..100);

    // println!("Please input a guess");

    // loop {
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please type a valid number !");
    //             continue;
    //         }
    //     };

    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small !"),
    //         Ordering::Greater => println!("Too big !"),
    //         Ordering::Equal => {
    //             println!("You win !");
    //             break;
    //         }
    //     }

    // }
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("Please input a guess");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number !");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You won !");
                break;
            }
        }
    }

    // prompt the user to guess a number,
    // loop that prompt untill the number is found.
    // Handle errors, such as Nan number input.

    // Prompt if the number is higher or lower, prompt if won
}
