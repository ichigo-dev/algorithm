/*

    010 - Factorial

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 1 && n <= 20);
    println!("{}", (1..(n+1)).product::<usize>());
}
