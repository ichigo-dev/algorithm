/*

    015 - Calculate GCD

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o

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

fn main()
{
    let inputs: Vec<usize> = input_vec();
    let (a, b) = (inputs[0], inputs[1]);
    assert!(a >= 1);
    assert!(b <= 10_usize.pow(9));

    println!("{}", gcd(a, b));
}
