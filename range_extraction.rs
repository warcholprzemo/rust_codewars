/*
A format for expressing an ordered list of integers is to use a comma separated list of either

* individual integers
* or a range of integers denoted by the starting integer separated from the end integer in the
  range by a dash, '-'. The range includes all integers in the interval including both endpoints.
  It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"
 */
mod solution {
    pub fn range_extraction(array: &[i32]) -> String {
        let array_len = array.len();
        let mut i: usize = 0;
        let mut result = vec![];
        while i < array_len {
            let el = array[i];
            if i < array_len - 2 && el == array[i+1] - 1 && el == array[i+2] - 2 {
                let mut max_index = i + 3;
                while max_index < array_len && array[max_index-1] == array[max_index] - 1 {
                    max_index += 1;
                }
                result.push(format!("{}-{}", el, array[max_index-1]));
                i = max_index;
            } else {
                result.push(el.to_string());
                i += 1;
            }
        }
        result.join(",")
    }
}

fn main() {
    let s1: String = solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]);
    let s2: String = solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]);
    let s3: String = solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,20,22]);
    let expected_s1: String = String::from("-6,-3-1,3-5,7-11,14,15,17-20");
    let expected_s2: String = String::from("-3--1,2,10,15,16,18-20");
    let expected_s3: String = String::from("-3--1,2,10,15,16,18,20,22");
    println!("{}, {}", s1, s1==expected_s1);
    println!("{}, {}", s2, s2==expected_s2);
    println!("{}, {}", s3, s3==expected_s3);
}
