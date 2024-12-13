use rand::{thread_rng, Rng};

const EXIT_FAILURE: i32 = 1;  // u8 is a byte
const EXIT_SUCCESS: i32 = 0;

// == GUESSER CONSTANTS ==
const LOWEST_GUESS: i32 = 0;
const HIGHEST_GUESS: i32 = 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // validate that number was passed in
    if args.len() < 2 {
        println!("Usage: ./number_guesser <guess>");
        std::process::exit(EXIT_FAILURE);
    }

    // parse guess from command line
    let guess = match args[1].parse::<i32>() {
        Ok(number) => number,
        Err(_e) => {
            println!("Invalid guess: guess must be a number between {low} and {high}", low=LOWEST_GUESS, high=HIGHEST_GUESS);
            std::process::exit(EXIT_FAILURE)
        }
    };

    if guess < LOWEST_GUESS || guess > HIGHEST_GUESS {
        println!("Invalid guess: guess must be a number between {low} and {high}", low=LOWEST_GUESS, high=HIGHEST_GUESS);
        std::process::exit(EXIT_FAILURE);
    }

    // generate the random number
    // TODO: check for set_target option and set target value to that instead of random value if given
    let mut rng = thread_rng();
    let target: i32 = rng.gen_range(LOWEST_GUESS..=HIGHEST_GUESS);

    // TODO: check for a debug option and print this if debug=True
    // println!("Target: {}", target);

    // see if guess was correct
    if guess == target {
        print!("You're correct! ");
    } else {
        print!("You're incorrect. ");
    }
    print!("The number was {}.\n", target);
    std::process::exit(EXIT_SUCCESS);
}
