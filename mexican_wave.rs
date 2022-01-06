/*
Create vector of mexican wave for given string.
If char is empty string then omit it.
For example:
"two words" ->
["Two words", "tWo words", "twO words", "two Words", "two wOrds", "two woRds", "two worDs", "two wordS"]
 */
fn wave(input_str: &str) -> Vec<String> {
    let mut result: Vec::<String> = Vec::new();
    for (i, letter) in input_str.to_lowercase().char_indices() {
        if letter == ' ' { continue; }
        result.push(format!(
            "{}{}{}",
            &input_str[..i],
            &letter.to_string().to_uppercase(),
            &input_str[i+1..]
        ));
    }
    result
}

fn main() {
    println!("{:?}", wave("hello"));
    println!("{:?}", wave("two words"));
    println!("{:?}", wave(" gap "));
}
