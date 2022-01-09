/*
Your task in order to complete this Kata is to write a function which formats a duration,
given as a number of seconds, in a human-friendly way.

The function must accept a non-negative integer. If it is zero, it just returns "now".
Otherwise, the duration is expressed as a combination of years, days, hours, minutes and seconds.

The resulting expression is made of components like 4 seconds, 1 year, etc.
In general, a positive integer and one of the valid units of time, separated by a space.
The unit of time is used in plural if the integer is greater than 1.

The components are separated by a comma and a space (", "). Except the last component,
which is separated by " and ", just like it would be written in English.

A more significant units of time will occur before than a least significant one.
Therefore, 1 second and 1 year is not correct, but 1 year and 1 second is.
*/

const YEAR: u64 = 365 * 24 * 60 * 60;
const DAY : u64 = 24 * 60 * 60;
const HOUR : u64 = 60 * 60;
const MINUTE : u64 = 60;

fn format_part(part_number: u64, part_str: &str, result: &mut Vec<String>) {
    if part_number == 1 {
        result.push(format!("1 {}", part_str));
    } else if part_number > 1 {
        result.push(format!("{} {}s", part_number, part_str));
    }
}

fn format_duration(secs: u64) -> String {
    if secs == 0 { return String::from("now"); }
    let mut seconds: u64 = secs;
    let mut result: Vec<String> = Vec::new();

    let years = seconds / YEAR;
    seconds -= years * YEAR;
    let days = seconds / DAY;
    seconds -= days * DAY;
    let hours = seconds / HOUR;
    seconds -= hours * HOUR;
    let minutes = seconds / MINUTE;
    seconds -= minutes * MINUTE;
    format_part(years, "year", &mut result);
    format_part(days, "day", &mut result);
    format_part(hours, "hour", &mut result);
    format_part(minutes, "minute", &mut result);
    format_part(seconds, "second", &mut result);

    let last_el = result.pop().unwrap();
    if result.is_empty() {
        last_el
    } else {
        format!("{} and {}", result.join(", "), last_el)
    }
}

fn main() {
    println!("{}", format_duration(3662)); // 1 hour, 1 minute and 2 seconds
    println!("{}", format_duration(0)); // now
}
