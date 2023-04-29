/*

    006 - Print 2N+3

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_f

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 1 && n <= 100);
    println!("{}", 2 * n + 3);
}
