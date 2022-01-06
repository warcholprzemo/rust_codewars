/*
for given vector of numbers return a sum that is <= `max_sum`
compute sum from every combination of subset with given `subset_length`
If the sum does not exist return -1
*/

fn make_sums_for_all_pairs(l: &Vec<i32>, pair: &Vec<i32>, subset_length: i32) -> Vec<i32> {
    if pair.len() as i32 == subset_length {
        return vec![pair.iter().sum()];
    }

    let mut all_sums: Vec<i32> = Vec::new();
    for (i, el) in l.iter().enumerate() {
        let mut new_pair = pair.to_vec();
        new_pair.push(*el);
        let mut local_sums = make_sums_for_all_pairs(&l[i + 1..].to_vec(), &new_pair, subset_length);
        all_sums.append(&mut local_sums);
    }
    all_sums
}

fn choose_best_sum(max_sum: i32, subset_length: i32, ls: &Vec<i32>) -> i32 {
    let mut solution: i32 = -1;
    for sum in make_sums_for_all_pairs(ls, &vec![], subset_length) {
        if sum <= max_sum && sum > solution {
            solution = sum;
        }
    }
    solution
}

fn main() {
    let ts = &vec![50, 55, 56, 57, 58];
    println!("SOLUTION {}", choose_best_sum(163, 3, ts)); // exp 163

    let ts = &vec![50];
    println!("SOLUTION {}", choose_best_sum(163, 3, ts));  // exp -1

    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    println!("SOLUTION {}", choose_best_sum(230, 3, ts));  // exp 228
    println!("SOLUTION {}", choose_best_sum(331, 2, ts));  // exp 178
}
