use std::fs;
use std::io;
use rand::Rng;

fn main() {
    // Load the wordlist
    let word_list = load_word_list("wordlist.txt".to_string());

    println!("Welcome to hangman!");

    // An array of booleans representing which letters have been guessed
    let mut guessed_chars: [bool; 26] = [false; 26];

    // Get word to guess
    let word = get_random_word(&word_list);
    let mut lives: u8 = 5;
    loop {


        // Prints only the letters guessed in the word and checks if the game has been won
        let mut finished = true;
        for c in word.chars() {
            if guessed_chars[(c as u8 - b'a' ) as usize] {
                print!("{c}");
            } else {
                print!("_");
                finished = false;
            }
        }
        print!("\n");

        // Checks if the game was won
        if finished {
            println!("You won! good job.");
            break;
        }

        // Prompt the user for input
        let guess = prompt_for_letter(&guessed_chars);
        // Flag the letter as guessed
        guessed_chars[(guess as u8 - b'a') as usize] = true;

        // Check if the letter is in the word
        let mut used_letter = 0;
        for c in word.chars() {
            if c == guess {
                used_letter += 1;
            }
        }

        // Adjust lives based on if the letter is in the word
        if used_letter == 0 {
            lives -= 1;
            println!("That letter is not in the word. You have lost one life.");
            println!("You have {lives} lives left.");

            // Check if the player has lost all of his or her lives
            if lives <= 0  {
                println!("You have run out of lives. \nYou lose");
                break;
            }
        } else {
            println!("That letter appeared in the word {used_letter} times.");
        }

    }

}

fn prompt_for_letter(guessed_chars: &[bool; 26]) -> char {
    println!("What letter would you like to guess next?");
    loop {
        // Get input from user
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Check all conditions:
        // Must be one character
        // Character must be a letter
        // Letter must not have been guessed yet
        if guess.chars().count() == 2 {
            guess = guess.to_lowercase();
            let c = guess.chars().nth(0).unwrap();
            let val: isize = c as isize - (b'a' as isize);
            if val < 0 || val > 26 {
                println!("Please type a letter.");
            } else {
                if guessed_chars[val as usize] {
                    println!("Please type a letter that has not been guessed yet.")
                } else {
                    return c;
                }
            }
        } else {
            println!("Please enter a single letter.");
        }
    }
}

/**
Struct containing all of the information of the wordlist
**/
struct WordList {
    words: Vec<String>,
    length: usize,
}

/**
Loads a word list at the file path given
**/
fn load_word_list(file_path: String) -> WordList {
    // Load file as a string from file_path
    let word_list_string: String = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    // Split the file string into individual words
    let words: Vec<String> = word_list_string.split("\n").map(str::to_string).collect();

    // Get the amount of words in the wordlist
    let length: usize = words.len();

    let word_list = WordList {
        words,
        length,
    };
    return word_list;
}

/**
Loads a random word from the given wordlist
**/
fn get_random_word(word_list: &WordList) -> String {
    let len = word_list.length;

    // Gets a random index within the bounds of the wordlist vector
    let index = rand::thread_rng().gen_range(1..= len);

    // Get the random word
    let word:String = word_list.words.as_slice()[index].clone();

    return word;
}