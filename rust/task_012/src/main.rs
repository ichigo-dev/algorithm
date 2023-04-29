/*

    012 - Primality Test

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l

*/

use algorithm_util::*;

fn is_prime( n: usize ) -> bool
{
    assert!(n > 1);
    let mut i = 3;
    while i * i < n
    {
        if n % i == 0
        {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 10_usize.pow(13));
    println!("{}", if is_prime(n) { "Yes" } else { "No" });
}
