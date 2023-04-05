use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a phrase to translate to Pig Latin:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let words = input.trim().split_whitespace();
    let mut pig_latin_words = Vec::new();

    for word in words {
        pig_latin_words.push(translate_word_to_pig_latin(word));
    }

    let pig_latin_phrase = pig_latin_words.join(" ");
    println!("Pig Latin: {}", pig_latin_phrase);
}

fn translate_word_to_pig_latin(word: &str) -> String {
    let first_letter = word.chars().next().unwrap();
    let rest_of_word = &word[1..];

    if is_vowel(first_letter) {
        format!("{}{}{}", rest_of_word, first_letter, "yay")
    } else {
        format!("{}{}{}", rest_of_word, first_letter, "ay")
    }
}

fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}
