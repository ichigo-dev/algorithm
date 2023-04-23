/*

    001 - Print 5+N

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_a

*/

use algorithm_util::*;

fn main()
{
    let apple_num: usize = 5;
    let orange_num: usize = input();

    assert!(1<=orange_num);
    assert!(orange_num<=100);

    println!("{}", apple_num + orange_num);
}
