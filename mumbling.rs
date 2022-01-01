/*
This time no story, no theory. The examples below show you how to write function accum:

accum("abcd") -> "A-Bb-Ccc-Dddd"
accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
accum("cwAt") -> "C-Ww-Aaa-Tttt"

The parameter of accum is a string which includes only letters from a..z and A..Z.
 */

fn accum(text :&str) -> String {
    if text.len() == 0 {
        return String::from("");
    }

    let mut n: usize = 0;
    let mut result = String::from("");
    for letter in text.to_lowercase().chars() {
        let upper_letter = letter.to_string().to_uppercase();
        let repeated_tail = letter.to_string().repeat(n);

        result = format!("{}-{}{}", &result, &upper_letter, &repeated_tail);
        n += 1;
    }
    result[1..].to_string()  //remove first '-'
}

fn main() {
    println!("{}", accum("RqaEzty"));
    println!("{}", accum("abcd"));
    println!("{}", accum(""));
}
