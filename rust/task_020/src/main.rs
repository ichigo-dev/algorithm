/*

    020 - Choose Cards 2

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t

*/

use algorithm_util::*;

fn main()
{
    let n: usize = input();
    assert!(n >= 2 && n <= 100);

    let cards: Vec<usize> = input_vec();
    assert!(cards.len() == n);

    //  全探索
    let s = 1000;
    let mut answer = 0;
    for i1 in 0..n
    {
        for i2 in (i1+1)..n
        {
            for i3 in (i2+1)..n
            {
                for i4 in (i3+1)..n
                {
                    for i5 in (i4+1)..n
                    {
                        let sum
                            = cards[i1]
                            + cards[i2]
                            + cards[i3]
                            + cards[i4]
                            + cards[i5];
                        if sum == s
                        {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", answer);
}
