/*

    018 - Convenience Store 1

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 200000);

    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == n);

    let mut items = (0, 0, 0, 0);
    for i in 0..n
    {
        match nums[i]
        {
            100 => { items.0 += 1; },
            200 => { items.1 += 1; },
            300 => { items.2 += 1; },
            400 => { items.3 += 1; },
            _ => {},
        }
    }
    println!("{}", items.0 * items.3 + items.1 * items.2);
}
