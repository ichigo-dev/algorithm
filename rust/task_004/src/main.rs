/*

    004 - Product of 3 Integers

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_d

*/

use algorithm_util::*;

fn main()
{
    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == 3);

    let mut product: usize = 1;
    for num in nums
    {
        assert!(num <= 100);
        product *= num;
    }

    println!("{}", product);
}
