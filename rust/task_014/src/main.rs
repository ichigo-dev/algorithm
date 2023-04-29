/*

    014 - Factorization

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_n

*/

use algorithm_util::*;

fn main()
{
    let mut n: usize = input();
    assert!(n >= 2 && n <= 10_usize.pow(12));

    let mut prime_factors = Vec::new();
    let mut i = 2;

    while i * i <= n
    {
        if n % i != 0
        {
            continue;
        }

        //  割り切れた回数を記録
        while n % i == 0
        {
            prime_factors.push(i.to_string());
            n /= i;
        }

        i += 1;
    }

    //  最後に素数が余ることがあるので、それも追加する
    if n != 1
    {
        prime_factors.push(n.to_string());
    }

    println!("{}", prime_factors.join(" "));
}
