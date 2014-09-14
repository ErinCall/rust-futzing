use std::io;
use std::rand::Rng;


fn main() {
    let secret_number = std::rand::task_rng().gen_range(1i, 101);
    let mut reader = io::stdin();
    let mut tries = 0i;

    println!("Ok, I'm thinking of a number in [1, 100]. Try and guess it!");

    loop {
        let input = reader.read_line().ok().expect("Failed to read line");
        let input_num: Option<int> = from_str(input.as_slice().trim());

        match input_num {
            None => println!("That's not even a number, dwag"),
            Some(num) => if num == secret_number {
                             println!("You got it!")
                             break
                         } else if num > secret_number {
                             tries += 1;
                             println!("You're high!")
                         } else {
                             tries += 1;
                             println!("You're low!")
                         }
        }
        if tries > 5 {
            println!("You ran out of guesses! The number was {:d}", secret_number)
            break
        }
    }
}
