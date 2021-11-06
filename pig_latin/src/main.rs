fn pig_latin(input: String) {
    let words = input.split_whitespace();
    for word in words {
        let mut chars = word.chars();
        let mut first_char = chars.next().unwrap().to_string();
        let mut rest = chars.collect::<String>();
        if ["a", "e", "i", "o", "u"].contains(&first_char.as_str()) {
            rest = word.to_string();
            first_char = "h".to_string();
        }
        print!("{}-{}{} ", rest, first_char, "ay");
    }
}

fn main() {
    pig_latin("Would you like some apples".to_string());
}
