use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec; // 0.7.2

pub fn get_user_input_length(question: &str, length: usize) -> std::string::String {
    let mut valid = false;
    let mut user_word: std::string::String = String::new();

    while !valid {
        user_word = get_user_input(question);
        if user_word.len() == length {
            valid = true;
        } else {
            println!(
                "The word has to be {} chars long, you entered {} chars",
                length,
                user_word.len()
            );
        }
    }

    return user_word;
}

pub fn get_user_input(question: &str) -> std::string::String {
    use std::io::{stdin, stdout, Write};

    let mut user_input = String::new();
    print!("{}", question);

    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_input)
        .expect("Did not enter a correct string");

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input;
}

pub fn get_words() -> std::vec::Vec<String> {
    let mut all_words = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./wordlist.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                //println!("{}", word);
                all_words.push(word)
            }
        }
    }

    return all_words;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_random_word() -> std::string::String {
    let all_words = get_words();
    let chosen_word = all_words.choose(&mut rand::thread_rng());
    //println!("{:?}", chosen_word);
    return chosen_word.unwrap().to_string();
}
