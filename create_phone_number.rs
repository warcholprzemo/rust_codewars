/*
Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
expected format: "(123) 456-7890"
*/

fn u8_array_to_str(buf: &[u8]) -> String {
    let mut result = String::from("");
    for element in buf.iter() {
        result += &element.to_string();
    }
    result
}

fn create_phone_number(numbers: &[u8]) -> String {
    let mut result = String::from("");
    result += "(";
    result += &u8_array_to_str(&numbers[0..3]);
    result += ") ";
    result += &u8_array_to_str(&numbers[3..6]);
    result += "-";
    result += &u8_array_to_str(&numbers[6..10]);
    result
}

fn main() {
    let result = create_phone_number(&[1,2,3,4,5,6,7,8,9,0]);
    println!("{}", result);
}
