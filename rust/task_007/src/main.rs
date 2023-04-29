/*

    007 - Number of Multiples 1

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_g

*/

use algorithm_util::*;

fn main()
{
    let inputs: Vec<usize> = input_vec();
    assert!(inputs.len() == 3);

    let (n, x, y) = (inputs[0], inputs[1], inputs[2]);
    assert!(n >= 1 && n <= 10_usize.pow(6));
    assert!(x >= 1 && x < y && y <= 10_usize.pow(6));

    let mut cnt = 0;
    for i in 1..(n+1)
    {
        if i % x == 0 || i % y == 0
        {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
