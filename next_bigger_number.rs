fn generate(k: usize, vec: &mut Vec<char>, combinations: &mut Vec<i64>, orig_n: i64) {
    if k == 1 {
        let combination_res = vec.iter().map(|char| {char.to_string()}).collect::<String>().parse::<i64>();
        let combination :i64 = match combination_res {
            Ok(number) => { number },
            Err(_) => { -1 }
        };
        if combination > orig_n {
            combinations.push(combination);
        }
    }
    else {
        generate(k-1, vec, combinations, orig_n);
        for i in 0..k-1 {
            if k % 2 == 0 {
                let temp = vec[i];
                vec[i] = vec[k-1];
                vec[k-1] = temp;
            } else {
                let temp = vec[0];
                vec[0] = vec[k-1];
                vec[k-1] = temp;
            }
            generate(k-1, vec, combinations, orig_n);
        }
    }
}

fn next_bigger_number(n: i64) -> i64 {
    let mut vec = n.to_string().chars().collect::<Vec<_>>();
    let mut combinations = Vec::new();
    generate(vec.len(), &mut vec, &mut combinations, n);
    combinations.sort();
    for el in combinations {
        if el > n {
            return el;
        }
    }
    -1
}

fn main(){
    println!("{}", next_bigger_number(12)); // 21
    println!("{}", next_bigger_number(513)); // 531
    println!("{}", next_bigger_number(2017)); // 2071
    println!("{}", next_bigger_number(414)); // 441
}
