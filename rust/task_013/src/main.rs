/*

    013 - Divisor Enumeration

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_m

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 10_usize.pow(13));

    let mut divisors = Vec::new();
    let mut i = 1;
    while i * i < n
    {
        if n % i != 0
        {
            continue;
        }

        divisors.push(i);
        if i != n / i
        {
            divisors.push(n/i);
        }

        i += 1;
    }

    divisors.sort();
    println!
    (
        "{}",
        divisors.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
