/*

    011 - Print Prime Numbers

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_k

*/

#![allow(dead_code)]
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
    assert!(n >= 2 && n <= 3000);

    //  全探索
    //
    //  let mut prime_numbers = Vec::new();
    //  for i in 2..(n+1)
    //  {
    //      if is_prime(i)
    //      {
    //          prime_numbers.push(i.to_string());
    //      }
    //  }
    //  println!("{}", prime_numbers.join(" "));

    //  エラトステネスの篩
    let mut prime_flags = vec![true; n];
    prime_flags[0] = false;
    prime_flags[1] = false;
    for i in 0..n
    {
        if prime_flags[i] == false
        {
            continue;
        }

        for j in (i*i..n).step_by(i)
        {
            prime_flags[j] = false;
        }
    }

    let mut prime_numbers = Vec::new();
    for (i, v) in prime_flags.iter().enumerate()
    {
        if *v
        {
            prime_numbers.push(i.to_string());
        }
    }
    println!("{}", prime_numbers.join(" "));
}
