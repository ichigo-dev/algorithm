/*

    009 - Brute Force 2

    https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i

*/

use algorithm_util::*;

fn main()
{
    let inputs: Vec<usize> = input_vec();
    assert!(inputs.len() == 2);

    let (n, s) = (inputs[0], inputs[1]);
    assert!(n >= 1 && n <= 60);
    assert!(s >= 1 && s <= 10000);

    let nums: Vec<usize> = input_vec();
    assert!(nums.len() == n);

    //  ビット全探索（計算量：2^N）
    //  * Nが大きくなると、計算量が指数的に増えていくので現実的ではない
    //
    //  ```
    //  let mut has_method = false;
    //  for bit in 0..(1_u64 << n)
    //  {
    //      let sum: usize = (0..n)
    //          .filter(|x| (bit & (1_u64 << x)) != 0)
    //          .map(|x| nums[x as usize])
    //          .sum();
    //
    //      if sum == s
    //      {
    //          has_method = true;
    //          break;
    //      }
    //  }
    //  println!("{}", if has_method { "Yes" } else { "No" });
    //  ```

    //  動的計画法(DP)（計算量：N*S）
    let mut dp_table = vec![vec![false; s+1]; n+1];
    dp_table[0][0] = true;
    for i in 0..n
    {
        for j in 0..(s+1)
        {
            if nums[i] > j
            {
                dp_table[i+1][j] = dp_table[i][j];
            }
            else
            {
                dp_table[i+1][j] = dp_table[i][j-nums[i]] || dp_table[i][j];
            }
        }
    }
    println!("{}", if dp_table[n][s] { "Yes" } else { "No" });
}
