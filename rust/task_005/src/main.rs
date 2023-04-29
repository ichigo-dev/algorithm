/*

    005 - Modulo 100

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_e

*/

use algorithm_util::*;

fn main()
{
    let input_size: usize = input();
    let nums: Vec<usize> = input_vec();
    assert!(input_size >= 1 && input_size <= 50 && nums.len() == input_size);

    let mut sum = 0;
    for num in nums
    {
        assert!(num >= 1 && num <= 100);
        sum += num;
    }

    println!("{}", sum % 100);
}
