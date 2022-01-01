use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let mut occurences = HashMap::new();
    for element in lst {
        let count = occurences.entry(element).or_insert(0);
        *count += 1;
        if *count <= n {
            result.push(*element);
        }
    }
    result
}

fn main() {
    println!("{:?}", delete_nth(&[20,37,20,21], 1));  // vec![20,37,21]);
    println!("{:?}", delete_nth(&[1,1,3,3,7,2,2,2,2], 3));  // vec![1, 1, 3, 3, 7, 2, 2, 2]);
}