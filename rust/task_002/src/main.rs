/*

    002 - Sum of 3 Integers

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_b

*/

use algorithm_util::*;

fn main()
{
    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == 3);

    let mut sum: usize = 0;
    for num in nums
    {
        assert!(num <= 100);
        sum += num;
    }

    println!("{}", sum);
}
