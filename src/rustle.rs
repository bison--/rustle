pub fn check_letters(word: &std::string::String, user: &std::string::String) -> bool {
    //println!("{:?}", word);

    let letter_modes = ["⬜", "🟨", "🟩"];
    let mut found_right_count = 0;

    // TODO: use foreach!
    for user_index in 0..5 {
        let mut found_letter_mode: usize = 0;

        for word_index in 0..5 {
            let user_char: u8 = user.as_bytes()[user_index];
            let word_char: u8 = word.as_bytes()[word_index];

            if user_char == word_char {
                if user_index == word_index {
                    found_letter_mode = 2;
                    found_right_count += 1;
                } else {
                    found_letter_mode = 1;
                }
            }
        }

        print!("{}", letter_modes[found_letter_mode]);
    }

    //println!("{}", found_right_count);
    println!();
    return found_right_count == 5;
}
