/*
The weight of a number will be from now on the sum of its digits.
When two numbers have the same "weight", let us class them as if they
were strings (alphabetical ordering) and not numbers.

Example
"56 65 74 100 99 68 86 180 90" ordered by numbers weights becomes:
"100 180 90 56 65 74 68 86 99"

180 and 90 have the same weight, but 180 is first in alphabetical order
*/
use std::collections::HashMap;

fn order_weight(input_str: &str) -> String {
    let mut final_result = Vec::new();
    let mut number_map: HashMap<i32, Vec<&str>> = HashMap::new();

    /* prepare the map
    { weight:u32 -> vec!["value1", "value2", ...] }
    */
    for number_str in input_str.split_whitespace() {
        let weight = number_str.chars().map(|char| char.to_digit(10).unwrap() as i32).sum();
        number_map.entry(weight).or_insert(Vec::new()).push(number_str);
    }

    /* sort by weights */
    let mut sorted_weight_numbers: Vec<(&i32, &Vec<&str>)> = number_map.iter().collect();
    sorted_weight_numbers.sort();

    /* sort alphabetically values for the same weight */
    for weight_numbers in sorted_weight_numbers {
        let mut sorted_numbers: Vec<&str> = weight_numbers.1.to_vec();
        sorted_numbers.sort();
        for number in sorted_numbers {
            final_result.push(number);
        }
    }

    final_result.join(" ")
}

fn main() {
    println!("{}", order_weight("56 65 74 100 99 68 86 180 90"));
}