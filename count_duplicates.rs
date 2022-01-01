use std::collections::HashSet;
use std::convert::TryInto;

fn count_duplicates(word: &str) -> u32 {
    let mut found_letters = Vec::new();
    let mut duplications = HashSet::new();

    for letter in word.to_lowercase().chars() {
        if found_letters.contains(&letter) {
            duplications.insert(letter);
        }
        else {
            found_letters.push(letter);
        }
    }
    println!("{:?}", duplications);
    duplications.len().try_into().unwrap()  // usize -> u32
}


fn main() {
    println!("{}", count_duplicates("abcdE")); // 0 - characters repeats more than once
    println!("{}", count_duplicates("Indivisibilities")); // 2 - # 'i' occurs seven times and 's' occurs twice
    println!("{}", count_duplicates("aabBcde")); // 2 - 'a' occurs twice and 'b' twice (`b` and `B`)
}
