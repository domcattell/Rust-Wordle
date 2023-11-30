use rand::Rng;
use std::fs;
use std::io;
use std::path::Path;

const WORDS_FILE_NAME: &str = "words.txt";
const ATTEMPTS: u8 = 10;

fn main() {
    let chosen_word: String = match get_word() {
        Some(unwrapped_word) => unwrapped_word,
        None => panic!("Error getting word"),
    };

    let word_to_guess_chars: Vec<char> = chosen_word.chars().collect();
    let mut attempts_left: u8 = ATTEMPTS;
    let mut word: [char; 5] = ['_', '_', '_', '_', '_'];
    let mut guessed_words: Vec<String> = vec![];

    while attempts_left > 0 {
        let mut guess: String = String::new();

        println!(
            "Word \x1b[48;5;17m\x1b[1m{}\x1b[0m, attempts left ({})",
            word.iter().collect::<String>(),
            attempts_left
        );

        if guessed_words.len() > 0 {
            println!("Words attempted");

            for guessed_word in guessed_words.iter() {
                println!("\x1b[31m{}\x1b[0m", guessed_word);
            }
        }

        println!("Enter your word");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_lowercase();

        if !is_in_words_list(guess.clone()) {
            println!("\x1b[33mWord not in words list!\x1b[0m");
            continue;
        }  
        
        if guessed_words.len() > 0 && guessed_words.iter().any(|x| x.clone() == guess) {
            println!("\x1b[33mWord already guessed!\x1b[0m");
            continue;
        }

        guessed_words.push(guess.clone());

        if is_in_word(guess.clone(), &mut word, &word_to_guess_chars) {
            if word.iter().all(|&x| x != '_') {
                break;
            }
        }
        
        attempts_left -= 1;
    }

    if attempts_left > 0 {
        println!("Correctly guessed word! \x1b[32m{}\x1b[0m", chosen_word);
        return;
    }

    println!("\x1b[31mFailed, word was {}", chosen_word)
}

fn is_in_word(guessed_word: String, word: &mut [char; 5], word_to_guess_chars: &Vec<char>) -> bool {
    let mut correct_guess = false;
    let mut count = 0;

    let guessed_word = guessed_word.to_lowercase();
    let guessed_words_char: Vec<char> = guessed_word[..5].chars().collect();

    loop {
        let guessed_word_char = guessed_words_char.get(count).unwrap().clone();
        let word_to_guess_char = word_to_guess_chars
            .get(count)
            .unwrap()
            .to_lowercase()
            .next()
            .unwrap();

        if guessed_word_char == word_to_guess_char && guessed_word_char != '_' {
            word[count] = guessed_word_char;
            correct_guess = true;
        }

        count += 1;

        if count == 5 {
            break;
        }
    }

    return correct_guess;
}

fn get_words() -> Vec<String> {
    let source_dir: &Path = Path::new(file!()).parent().unwrap();
    let file_path: std::path::PathBuf = source_dir.join(WORDS_FILE_NAME);

    let file_content: String = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading the file: {}", err);
        }
    };

    return file_content
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
}

fn get_word() -> Option<String> {
    let words_vec: Vec<String> = get_words();

    let random_word_index: usize = rand::thread_rng().gen_range(0..=(words_vec.len() - 1));

    if let Some(word) = words_vec.get(random_word_index) {
        return Some(word.clone().trim().to_string());
    }

    return None;
}

fn is_in_words_list(guessed_word: String) -> bool {
    let mut contains = false;

    for word in get_words().iter() {
        if word.to_lowercase() == guessed_word {
            contains = true;
            break;
        }
    }

    return contains;
}
