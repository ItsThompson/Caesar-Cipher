use std::{isize, usize};

trait CaesarCipher {
    fn get_alphabet(&self) -> [char; 26];
    fn get_index_of_alphabet(&self, c: char) -> usize;
    fn validate_encrypted_message(&self) -> bool;
    fn sanatize_plain_message(&self) -> String;
    fn shift_characters(&self, shift: isize) -> String;
    fn encrypt(&self, shift: isize) -> String;
    fn decrypt(&self, shift: isize) -> String;
}

impl CaesarCipher for String {
    fn get_alphabet(&self) -> [char; 26] {
        [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ]
    }

    fn get_index_of_alphabet(&self, c: char) -> usize {
        match c.to_ascii_lowercase() {
            'a'..='z' => c.to_ascii_lowercase() as usize - 'a' as usize,
            _ => 0,
        }
    }

    fn sanatize_plain_message(&self) -> String {
        let string = self.to_uppercase();
        let mut output: String = String::new();

        for character in string.chars() {
            if character.is_alphabetic() || character.is_whitespace() {
                output.push(character)
            }
        }
        output
    }

    fn shift_characters(&self, shift: isize) -> String {
        let alphabet: &[char; 26] = &self.get_alphabet();
        let plain_message: String = self.sanatize_plain_message();

        let mut output: String = String::new();

        for character in plain_message.chars() {
            let mut new_char: char = ' ';

            if character == new_char {
                output.push(new_char);
                continue;
            }

            let char_index: isize = self.get_index_of_alphabet(character).try_into().unwrap();
            let alphabet_length: isize = alphabet.len().try_into().unwrap();

            let new_unmod_index: isize = char_index + shift;

            let new_char_index: usize = new_unmod_index.rem_euclid(alphabet_length).try_into().unwrap();

            new_char = alphabet[new_char_index];
            output.push(new_char);
        }
        output
    }

    fn encrypt(&self, shift: isize) -> String {
        self.shift_characters(shift)
    }

    fn validate_encrypted_message(&self) -> bool {
        true
    }

    fn decrypt(&self, shift: isize) -> String {
        self.shift_characters(-shift)
    }
}

fn main() {
    let plain_text: String = String::from("Hello World");
    println!("show {}", plain_text.encrypt(5));

    let encrypted_text: String = String::from("MJQQT BTWQI");
    println!("show {}", encrypted_text.decrypt(5));
}
