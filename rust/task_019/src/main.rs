/*

    019 - Choose Cards 1

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 500000);

    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == n);

    let mut cards = (0, 0, 0);
    for i in 0..n
    {
        match nums[i]
        {
            1 => { cards.0 += 1 },
            2 => { cards.1 += 1 },
            3 => { cards.2 += 1 },
            _ => panic!("Invalid input"),
        }
    }
    let result
        = ( cards.0 * (cards.0 - 1) / 2 )
        + ( cards.1 * (cards.1 - 1) / 2 )
        + ( cards.2 * (cards.2 - 1) / 2 );
    println!("{}", result);
}
