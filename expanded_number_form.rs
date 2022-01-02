/*
Write Number in Expanded Form
For example
70304 -> "70000 + 300 + 4"
 */

fn expanded_form(start_n: u64) -> String {
    let mut n: u64 = start_n;
    let mut pow: u32 = 1;
    let mut vec = Vec::new();
    let mut rest: u64;
    while n > 0 {
        rest = n % (10_u64.pow(pow));
        if rest > 0 {
            vec.push(rest.to_string());
        }
        pow += 1;
        n = n - rest;
    }
    vec.reverse();
    vec.join(" + ")
}

fn main() {
    println!("{}", expanded_form(12)); // 10 + 2
    println!("{}", expanded_form(42)); // 40 + 2
    println!("{}", expanded_form(70304)); // 70000 + 300 + 4
}
