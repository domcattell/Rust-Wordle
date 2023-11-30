use rand::Rng;
use std::fs;
use std::io;
use std::path::Path;

const WORDS_FILE_NAME: &str = "words.txt";

fn is_in_word(guessed_word: String, word: &mut [char; 5], word_to_guess_chars: &Vec<char>) -> bool {
    let mut correct_guess = false;
    let guessed_words_char: Vec<char> = guessed_word[..5].chars().collect();
    let mut count = 0;

    loop {
        let guessed_word_char = guessed_words_char.get(count).unwrap().to_lowercase().next().unwrap();
        let word_to_guess_char = word_to_guess_chars.get(count).unwrap().to_lowercase().next().unwrap();

        if guessed_word_char == word_to_guess_char {
            correct_guess = true;
            word[count] = guessed_word_char;
        }

        count += 1;

        if count == 5 {
            break;
        }
    }

    return correct_guess
}

fn get_word() -> Option<String> {
    let source_dir = Path::new(file!()).parent().unwrap();
    let file_path = source_dir.join(WORDS_FILE_NAME);

    let file_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
            return None;
        }
    };

    let words_vec: Vec<String> = file_content
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let random_word_index = rand::thread_rng().gen_range(0..=(words_vec.len() - 1));
    
    if let Some(word) = words_vec.get(random_word_index) {
        return Some(word.clone());
    }

    return None;
}

fn main() {
    let chosen_word: String = match get_word() {
        Some(unwrapped_word) => unwrapped_word,
        None => panic!("Error getting word")
    };

    let word_to_guess_chars: Vec<char> = chosen_word.chars().collect();
    let mut attempts_left = 5i8;
    let mut word: [char; 5] = ['_', '_', '_', '_', '_'];

    while attempts_left > 0 {
        println!("Choose a five letter word");
        println!("Word {}, attempts left ({})", word.iter().collect::<String>(), attempts_left);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if is_in_word(guess, &mut word, &word_to_guess_chars) {
            if word.iter().all(|&x| x != '_') {
                break;
            }

            continue;
        }

        attempts_left -= 1;
    }

    if attempts_left > 0 {
        println!("Correctly guessed word! {}", chosen_word);
        return;
    }

    println!("Failed, word was {}", chosen_word)
}
