/*
Implement a function that receives two IPv4 addresses, and returns the number of
addresses between them (including the first one, excluding the last one).

All inputs will be valid IPv4 addresses in the form of strings.
The last address will always be greater than the first one.

ips_between("10.0.0.0", "10.0.0.50") ==  50
ips_between("10.0.0.0", "10.0.1.0")  == 256
ips_between("20.0.0.10", "20.0.1.0") == 246
*/

use std::convert::TryInto;

fn ip_to_number(ip: &str) -> u32 {
    let parts: Vec<u32> = ip.split(".").map(|s| s.parse::<u32>().unwrap()).collect();
    let mut number: u32 = 0;
    for (i, part) in parts.iter().enumerate() {
        number += part * 256_u32.pow((3 - i).try_into().unwrap())
    }
    number
}

fn ips_between(start: &str, end: &str) -> u32 {
    ip_to_number(end) - ip_to_number(start)
}

fn main() {
    println!("{}", ips_between("20.0.0.10", "20.0.1.0"))
}