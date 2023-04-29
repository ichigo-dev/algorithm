/*

    008 - Brute Force 1

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h

*/

use algorithm_util::*;

fn main()
{
    let inputs: Vec<usize> = input_vec();
    assert!(inputs.len() == 2);

    let (n, s) = (inputs[0], inputs[1]);
    assert!(n >= 1 && n <= 1000);
    assert!(s >= 1 && s <= 2000);

    let mut cnt = 0;
    for i in 1..=n
    {
        for j in 1..=n
        {
            if i + j <= s
            {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
