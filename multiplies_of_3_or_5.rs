/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Finish the solution so that it returns the sum of all the multiples of 3 or 5
below the number passed in.
Additionally, if the number is negative, return 0 (for languages that do have them).

Note: If the number is a multiple of both 3 and 5, only count it once.
 */

fn solution(num: i32) -> i32 {
    let mut numbers = Vec::new();
    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            numbers.push(i);
        }
    }
    numbers.iter().sum()
}

fn main() {
    println!("{}", solution(10));  // 23
    println!("{}", solution(11));  // 33
    println!("{}", solution(6));  // 8
    println!("{}", solution(1));  // 0
    println!("{}", solution(-5));  // 0
}
