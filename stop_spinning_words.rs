/*
Write a function that takes in a string of one or more words, and returns the same string,
but with all five or more letter words reversed (Just like the name of this Kata).
Strings passed in will consist of only letters and spaces.
Spaces will be included only when more than one word is present.

Examples:
spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw"
spinWords( "This is a test") => returns "This is a test"
spinWords( "This is another test" )=> returns "This is rehtona test"
 */

fn spin_words(words: &str) -> String {
    let mut vec = Vec::new();
    for word in words.split(" ") {
        if word.chars().count() >= 5 {
            vec.push(word.chars().rev().collect::<String>());
        }
        else {
            vec.push(word.to_string());
        }

    }
    vec.join(" ")
}

fn main() {
    println!("{}", spin_words("This is another test"));
    println!("{}", spin_words("buba"));
    println!("{}", spin_words("kabababa"));
}
