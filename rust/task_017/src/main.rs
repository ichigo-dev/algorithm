/*

    017 - Least Common Multiple of N Integers

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_q

*/

use algorithm_util::*;

//------------------------------------------------------------------------------
//  ユークリッドの互除法を用いた最大公約数の算出
//------------------------------------------------------------------------------
fn gcd( a: usize, b: usize ) -> usize
{
    let mut a = a;
    let mut b = b;

    while a >= 1 && b >= 1
    {
        if a < b { b = b % a; }
        else { a = a % b; }
    }

    if a >= 1 { return a; }
    else { return b; }
}

//------------------------------------------------------------------------------
//  最小公倍数の算出
//------------------------------------------------------------------------------
fn lcm( a: usize, b: usize ) -> usize
{
    a * b / gcd(a, b)
}

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 10_usize.pow(5));

    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == n);

    let mut result = nums[0];
    for i in 1..n
    {
        result = lcm(result, nums[i]);
    }
    println!("{}", result);
}
