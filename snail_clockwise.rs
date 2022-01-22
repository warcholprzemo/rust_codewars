/*
Given an n x n array, return the array elements arranged from outermost elements
to the middle element, traveling clockwise.
array = [[1,2,3],
         [4,5,6],
         [7,8,9]]
snail(array) #=> [1,2,3,6,9,8,7,4,5]
*/

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return vec![];
    }
    let n = matrix.len();
    let mut counter: usize = 0;
    let mut result = vec![];
    let max_i = n - 1;

    let mut step: usize = 0;
    loop {
        for i in step..n-step { result.push(matrix[step][i]); counter+=1; }
        if counter == n*n { break; }
        for i in step+1..n-step { result.push(matrix[i][max_i-step]); counter+=1; }
        for i in step+1..n-step { result.push(matrix[max_i-step][max_i-i]); counter+=1; }
        if counter == n*n { break; }
        for i in step+1..n-step-1 { result.push(matrix[max_i-i][step]); counter+=1; }
        step += 1;
    }
    result
}

fn main() {
    let square = &[
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12],
        vec![13,14,15,16],
    ];
    println!("{:?}", snail(square));
    let square = &[
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    println!("{:?}", snail(square));
    let square = &[
        vec![1],
    ];
    println!("{:?}", snail(square));
}