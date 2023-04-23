#![allow(dead_code)]

//------------------------------------------------------------------------------
//  Reads a line from standard input and converts it to the specified type.
//------------------------------------------------------------------------------
pub fn input<T: std::str::FromStr>() -> T
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

//------------------------------------------------------------------------------
//  Reads a line from standard input and converts it to a `Vec` of the specified
//  type with space delimiters.
//------------------------------------------------------------------------------
pub fn input_vec<T: std::str::FromStr>() -> Vec<T>
{
    input::<String>().split_whitespace()
        .map(|elm| elm.parse().ok().unwrap()).collect()
}

//------------------------------------------------------------------------------
//  Reads multiple lines from standard input and converts them to a
//  tow-dimensional `Vec` of specified type.
//------------------------------------------------------------------------------
pub fn input_matrix<T: std::str::FromStr>( n: usize ) -> Vec<Vec<T>>
{
    (0..n).map(|_| input_vec()).collect()
}
