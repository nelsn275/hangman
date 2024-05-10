use rand::{Rng, random};
use std::io;

fn main() {
    /* Run the hangman game */
    println!("\nWelcome to hangman!");

    // Define variables
    let mut counter = 1;
    let solution = retrieve_word();
    let mut progress = "_".repeat(solution.len());

    // Run first iteration of game since rust does not have a do while loop
    println!("Your word is {} letters long. Please guess one letter at a time", progress.len());
    println!("{}", progress);
    let mut guess = get_guess();

    progress = check_guess(guess, progress, &solution);

    // If the game is not over, keep having them guess letters until it is
    while !is_done(&progress, &solution){
        println!("\nHere is your progress: {}", progress);

        guess = get_guess();
        progress = check_guess(guess, progress, &solution);
        counter += 1;
    }

    // When the game is over display a congratulatory message
    println!("\nCongratulations! You have guessed the word {} ", progress);
    // Display the number of guesses it took
    println!("It took you {} guesses", counter);

    // If desired, the option to play again with a different word is offered
    println!("\nPress 1 to play again with a new word. Otherwise press 0 to end the program");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let again = input.to_lowercase().trim().chars().next().expect("No input received");

    if again == '1'{
        main();
    }

}

fn get_guess() -> char {
    /* Get a letter from the user */
    println!("What is your next guess? ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let guess = input.to_lowercase().trim().chars().next().expect("No input received");

    return guess;
}

fn check_guess(guess:char, mut progress:String, solution:&String) ->String {
    /* Take a letter and a word and determine if the letter is in the word */
    let mut i = 0;
    // Turn the solution and the progress into a list of chars so we can iterate through them using indicies
    let solution_letters: Vec<char> = solution.chars().collect();

    // Loop through all the letters and check to see if they match the guess. If they do, call update_progress
    while i < solution_letters.len() {
        if solution_letters[i] == guess {
            progress = update_progress(guess, i, progress);
        }

        i += 1;
    }
    return progress;
}

fn update_progress(guess:char, i:usize, progress:String) -> String {
    /* Replace the desired slot in progress with the correct letter */

    // Convert progress to a list of letters
    let mut progress_letters: Vec<char> = progress.chars().collect();

    // Change the letter at index i into the guess
    progress_letters[i] = guess;

    // Turn the list of letters back into a string
    let progress = progress_letters.into_iter().collect();

    // Tell the user they guessed correctly
    println!("Good guess! {} is in the word", guess);

    return progress
}

fn retrieve_word() -> String {
    /* Generate a random word to be used in hangman */

    // Preset list of words that could be used
    let words = vec!["window", "hammer", "purple", "planet", "bottle", "forest", "sunset", "pillow", "guitar"];
    // Get a random number to use as an index
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..words.len());


    return String::from(words[random_number]);
}

fn is_done(progress:&String, solution:&String) -> bool {
    /* Return a boolean value if the mystery word has been guessed or not */
    return progress == solution;
}

