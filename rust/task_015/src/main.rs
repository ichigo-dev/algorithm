/*

    015 - Calculate GCD

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o

*/

use algorithm_util::*;

fn gcd( a: usize, b: usize ) -> usize
{
    let mut answer = 0;
    for i in 1..(std::cmp::min(a, b)+1)
    {
        if a % i == 0 && b % i == 0
        {
            answer = i;
        }
    }
    answer
}

fn main()
{
    let inputs: Vec<usize> = input_vec();
    let (a, b) = (inputs[0], inputs[1]);
    assert!(a >= 1);
    assert!(b <= 10_usize.pow(9));

    println!("{}", gcd(a, b));
}
