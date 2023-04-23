/*

    003 - Sum of N Integers

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_c

*/

use algorithm_util::*;

fn main()
{
    let quantity: usize = input();
    assert!(1 <= quantity && quantity <= 50);

    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == quantity);

    let mut sum: usize = 0;
    for num in nums
    {
        assert!(num <= 100);
        sum += num;
    }

    println!("{}", sum);
}
