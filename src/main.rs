mod helper;
mod rustle;

fn main() {
    println!("
    **********
    * RUSTLE *
    **********"
    );

    println!("
    A simple WORDLE clone in Rust.
    You have to guess the WORD in six goes or less.
    A correct letter turns green 🟩.
    A correct letter in the wrong place turns yellow 🟨.
    An incorrect letter turns gray ⬜.
    Letters can be used more than once."
    );

    println!();

    let random_word = helper::get_random_word();
    //println!("{}", &random_word);

    let mut won = false;
    for _ in 0..5 {
        let user_word = helper::get_user_input_length("GUESS: ", 5);
        won = rustle::check_letters(&random_word, &user_word);
        if won {
            break;
        }
    }

    if won {
        println!("YOU WON 🎉");
    } else {
        println!("YOU LOST! 😞 The word was: '{}'", random_word);
    }

    //println!("{}", user_word);
}
