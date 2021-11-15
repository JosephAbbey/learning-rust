fn pig_latin(input: String) {
    let words = input.split_whitespace();
    // loop for each word in the string
    for word in words {
        // get a list of the characters in the word
        let mut chars = word.chars();
        // get the first character
        let mut first_char = chars.next().unwrap().to_string();
        // get the rest of the characters
        let mut rest = chars.collect::<String>();
        // is the first character a vowel?
        if ["a", "e", "i", "o", "u"].contains(&first_char.as_str()) {
            // if so, just add back the first character
            rest = word.to_string();
            // and set the first character to "h"
            first_char = "h".to_string();
        }
        print!("{}-{}{} ", rest, first_char, "ay");
    }
}

fn main() {
    pig_latin("Would you like some apples".to_string());
}
