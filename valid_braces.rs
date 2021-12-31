/*
All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.
What is considered Valid?

A string of braces is considered valid if all braces are matched with the correct brace.
"(){}[]"   =>  True
"([{}])"   =>  True
"(}"       =>  False
"[(])"     =>  False
"[({})](]" =>  False
*/

fn valid_braces(s: &str) -> bool {
    println!("****** Check {} ******", s);
    let good_pair1 = (Some('('), Some(')'));
    let good_pair2 = (Some('['), Some(']'));
    let good_pair3 = (Some('{'), Some('}'));

    let mut new_s: &str = s;
    let mut tmp_s: String;

    loop {
        if new_s.len() == 0 {
            break;
        }
        let mut i = 0;
        let n = new_s.len() - 1;
        let mut found_pair = false;

        while i < n {
            let pair = (new_s.chars().nth(i), new_s.chars().nth(i+1));
            if pair == good_pair1 || pair == good_pair2 || pair == good_pair3 {
                tmp_s = format!("{}{}", &new_s[..i], &new_s[i+2..]);
                new_s = &tmp_s;
                found_pair = true;
                break
            }
            i += 1;
        }
        if found_pair == false {
            return false;
        }
    }
    return true;
}

fn main(){
    println!("Result {}", valid_braces("()[]"));
    println!("Result {}", valid_braces("([{}]){}[[{}]]"));
    println!("Result {}", valid_braces("([){}"));
}
