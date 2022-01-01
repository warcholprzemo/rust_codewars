/*
Duplicate encoder

The goal of this exercise is to convert a string to a new string where each character
in the new string is "(" if that character appears only once in the original string, or ")"
if that character appears more than once in the original string. Ignore capitalization when
determining if a character is a duplicate.
*/


use std::collections::HashMap;

fn duplicate_encode(word:&str) -> String {
    let mut final_s = String::from("");
    let mut scores = HashMap::new();
    let mut letters = Vec::new();

    for c in word.chars() {
        let lower_char = c.to_lowercase();
        let lower_c1 = lower_char.to_string();
        let lower_c2 = lower_char.to_string();

        let count = scores.entry(lower_c1).or_insert(0);
        *count += 1;
        letters.push(lower_c2);
    }

    for value in &letters {
        let score = scores.get(value);

        match score {
            Some(el) => {
                if *el == 1 {
                    final_s += &String::from("(");
                }
                else {
                    final_s += &String::from(")");
                }
            },
            None => (),
        }
    }
    final_s
}


fn main() {
    println!("{}", duplicate_encode("Success"));
    println!("{}", duplicate_encode("din"));
    println!("{}", duplicate_encode("recede"));
    println!("{}", duplicate_encode("(( @"));
}
