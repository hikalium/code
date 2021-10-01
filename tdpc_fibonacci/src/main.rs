#![feature(map_first_last)]
use std::collections::HashMap;
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

/*

fn companion(n: usize) -> Vec<Vec<u64>> {
    let mut vec: Vec<Vec<u64>> = Vec::with_capacity(n);
    let mut row = Vec::new();
    row.resize(n, 1);
    vec.push(row);
    for i in 1..n {
        let mut row = Vec::new();
        row.resize(n, 0);
        row[i - 1] = 1;
        vec.push(row);
    }
    vec
}

fn unit(n: usize) -> Vec<Vec<u64>> {
    let mut vec: Vec<Vec<u64>> = Vec::with_capacity(n);
    let mut row: Vec<u64> = Vec::new();
    for i in 0..n {
        let mut row = Vec::new();
        row.resize(n, 0);
        row[i] = 1;
        vec.push(row);
    }
    vec
}

fn companion_pow(n: usize, k: usize) -> Vec<Vec<u64>> {
    let mut cm_result = unit(n);
    let mut cm = companion(n);

    let mut t = 0;
    println!("companion_pow n={} k={}", n, k);
    for i in 0..32 {
        if (k ^ t) == 0 {
            break;
        }
        println!("companion_pow {} {}", i, t);
        if (k & (1 << i)) != 0 {
            cm_result = mul_matrix_matrix(&cm_result, &cm);
            t |= (1 << i);
        }
        cm = mul_matrix_matrix(&cm, &cm);
    }
    cm_result
}

fn print_matrix(matrix: &Vec<Vec<u64>>) {
    for row in matrix {
        for v in row {
            print!("{} ", v);
        }
        println!("");
    }
    println!("");
}

fn print_vec(vec: &Vec<u64>) {
    for v in vec {
        print!("{} ", v);
    }
    println!("");
    println!("");
}

fn mul_matrix_vec(m: &Vec<Vec<u64>>, v: Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::with_capacity(v.len());
    result.resize(v.len(), 0);
    for i in 0..v.len() {
        let mut sum = 0;
        for k in 0..v.len() {
            sum += v[k] * m[i][k];
        }
        result[i] = sum % MOD;
    }
    result
}

fn mul_matrix_matrix(m1: &Vec<Vec<u64>>, m2: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    // returns m1 * m2
    let mut vec: Vec<Vec<u64>> = Vec::with_capacity(m1.len());
    for y in 0..m1.len() {
        let mut row = Vec::new();
        row.resize(m1.len(), 0);
        vec.push(row);
    }
    for y in 0..m1.len() {
        for x in 0..m1.len() {
            for k in 0..m1.len() {
                vec[y][x] = (vec[y][x] + m1[y][k] * m2[k][x]) % MOD;
            }
        }
    }
    vec
}

fn solution_naive(n: usize, k: usize) -> u64 {
    let mut kv: [u64; 1000 + 1] = [0; 1000 + 1];
    let mut next_value: u64 = 0;
    for i in 0..k {
        kv[i] = 1;
        next_value += 1;
    }
    for i in k..n {
        let ir = i % k;
        let next_next_value = (next_value + next_value) % 1_000_000_007;
        let next_next_value = if next_next_value > kv[ir] {
            next_next_value - kv[ir]
        } else {
            next_next_value + 1_000_000_007 - kv[ir]
        };
        kv[ir] = next_value;
        next_value = next_next_value;
    }
    return kv[(n - 1) % k];
}

fn solution_companion_mat(n: usize, k: usize) -> u64 {
    let cm = companion(k);

    let mut v = Vec::new();
    v.resize(k, 1);

    for _ in v.len()..n {
        v = mul_matrix_vec(&cm, v);
    }
    v[0]
}

fn solution_companion_mat2(n: usize, k: usize) -> u64 {
    if (n < k) {
        return 1;
    }
    let cm = companion_pow(k, n - k);

    let mut v = Vec::new();
    v.resize(k, 1);

    v = mul_matrix_vec(&cm, v);
    v[0]
}

use std::collections::HashMap;

fn calc_coefficients(n: usize, k: usize, c_cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if n < k {
        let mut c = vec![0; k];
        c[n] = 1;
        return c;
    }
    if n == k {
        return vec![1; k];
    }
    if c_cache.contains_key(&n) {
        return c_cache[&n].clone();
    }
    if n % 2 == 0 {
        let mid = n / 2;
        let mut mid_values = vec![vec![0; k]; k];
        for i in 0..k {
            mid_values[i] = calc_coefficients(mid + i, k, c_cache);
        }
        let adv = mid_values[0].clone(); // f(mid)
        let mut c = vec![0; k];
        for i in 0..k {
            for t in 0..k {
                c[i] = (c[i] + adv[i] * mid_values[t][i]) % (MOD as usize)
            }
        }
        c_cache.insert(n, c.clone());
        println!("A: {} = {:?}", n, c);
        c
    } else {
        let mut mid_values = vec![vec![0; k]; k];
        for i in 0..k {
            mid_values[i] = calc_coefficients(n - k + i, k, c_cache);
        }
        let adv = vec![1; k]; // f(1)
        let mut c = vec![0; k];
        for i in 0..k {
            for t in 0..k {
                c[i] = (c[i] + adv[i] * mid_values[t][i]) % (MOD as usize)
            }
        }
        c_cache.insert(n, c.clone());
        println!("B: {} = {:?}", n, c);
        c
    }
}
*/

/*
fn calc_coefficients_bottomup(
    n: usize,
    k: usize,
    c_cache: &mut HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    let c = vec![1; k]; // f(1)
    if n < k {
        let mut c = vec![0; k];
        c[n] = 1;
        return c;
    }
    if n == k {
        return vec![1; k];
    }
    if c_cache.contains_key(&n) {
        return c_cache[&n].clone();
    }
    if n % 2 == 0 {
        let mid = n / 2;
        let mut mid_values = vec![vec![0; k]; k];
        for i in 0..k {
            mid_values[i] = calc_coefficients(mid + i, k, c_cache);
        }
        let adv = mid_values[0].clone(); // f(mid)
        let mut c = vec![0; k];
        for i in 0..k {
            for t in 0..k {
                c[i] = (c[i] + adv[i] * mid_values[t][i]) % (MOD as usize)
            }
        }
        c_cache.insert(n, c.clone());
        println!("A: {} = {:?}", n, c);
        c
    } else {
        let mut mid_values = vec![vec![0; k]; k];
        for i in 0..k {
            mid_values[i] = calc_coefficients(n - k + i, k, c_cache);
        }
        let mut c = vec![0; k];
        for i in 0..k {
            for t in 0..k {
                c[i] = (c[i] + adv[i] * mid_values[t][i]) % (MOD as usize)
            }
        }
        c_cache.insert(n, c.clone());
        println!("B: {} = {:?}", n, c);
        c
    }
}
*/

/*
fn solution_kitamasa(n: usize, k: usize) -> u64 {
    let mut f_cache = HashMap::new();
    f_cache.insert(1, vec![vec![1; k]);
    let mut f = vec![1; k];
    let mut v = vec![1; k];
    let mut i = 1;
    while (i < k) {
        i *= 2;
    }
    println!("{:?}", f_cache);
    0
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let ev: Vec<&str> = rows[0].split(' ').collect();
    let k = ev[0].parse::<usize>().unwrap();
    let n = ev[1].parse::<usize>().unwrap();

    println!("{}", solution_kitamasa(n, k));

    Ok(())
}
*/

/*
k = 3
n:                    0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,    12,    13,    14,    15,    16,    17,    18,    19,    20,
v[n]:                 1,     1,     1,     3,     5,     9,    17,    31,    57,   105,   193,   355,   653,  1201,  2209,  4063,  7473, 13745, 25281, 46499, 85525,

w[n] = [v[n], v[n+1], v[n+2]]
w[0]                 [1,     1,     1]
w[1]                        [1,     1,     3]

v[n+i] = w[n] * f[i]

def:
f[0] = [1, 0, 0]
f[1] = [0, 1, 0]
f[2] = [0, 0, 1]
f[3] = [1, 1, 1]

v[3] = w[0] * f[3]
v[4] = w[0] * f[4]
     = w[1] * f[3]
     = [v[1], v[2], v[3]] * f[3]
     = [v[1], v[2], w[0] * f[3]] * f[3]

v[n] = w[0] * f[n] ...... *A
v[n+1] = w[1] * f[n]
v[n+2] = w[2] * f[n]
w[n] = [w[0] * f[n], w[1] * f[n], w[2] * f[n]]


# f[n] -> f[n+1]
v[n] = w[0] * f[n]
v[n+1] = w[1] * f[n]
       = [v[1], v[2], v[3]] * f[n]
       = [v[1], v[2], v[0] + v[1] + v[2]] * f[n]
       = v[1]*f[n][0] + v[2]*f[n][1] + (v[0] + v[1] * v[2]) * f[n][2]
       = w[0] * [f[n][2], f[n][0] + f[n][2], f[n][1] + f[n][2]]
       = w[0] * f[n+1]
->
f[n+1] = [f[n][2], f[n][0] + f[n][2], f[n][1] + f[n][2]] ..... *next

ex: f[2] -> f[3]
f[2] = [0, 0, 1]
f[3] = [1, 0 + 1, 0 + 1]
     = [1, 1, 1]


# f[n] -> f[2*n]
v[2*n] = w[0] * f[2*n]   ... *1
       = w[n] * f[n]
       = [w[0] * f[n], w[1] * f[n], w[2] * f[n]] * f[n]
       = [w[0] * f[n], w[0] * f[n+1], w[0] * f[n+2]] * f[n]
       = w[0] * f[n] * f[n][0] + w[0] * f[n+1] * f[n][1] + w[0] * f[n+2] * f[n][2]]
       = w[0] * (f[n] * f[n][0] + f[n+1] * f[n][1] + f[n+2] * f[n][2])    ... from *1
->
f[2*n] = (f[n] * f[n][0] + f[n+1] * f[n][1] + f[n+2] * f[n][2]) ..... *double

ex: f[3], f[4], f[5] -> f[6]
f[1] = [0, 1, 0]
f[2] = [0, 0, 1]
f[3] = [1, 1, 1]
f[4] = [1, 2, 2]
f[5] = [2, 3, 4]

f[6] = [4, 6, 7]

f[6] = f[3] * f[3][0] + f[4] * f[3][1] + f[5] * f[3][2]
     = [1, 1, 1] * 1 + [1, 2, 2] * 1 + [2, 3, 4] * 1
     = [4, 6, 7]


# combination of *next and *double
ex: f[6], f[7], f[8] -> f[12]
f[6] = [4, 6, 7]
f[7] = [7, 11, 13]
f[8] = [13, 20, 24]

f[12] = f[6] * f[6][0] + f[7] * f[6][1] + f[8] * f[6][2]
      = [4, 6, 7] * 4 + [7, 11, 13] * 6 + [13, 20, 24] * 7
      = [16, 24, 28] + [42, 66, 78] + [91, 140, 168]
      = [149, 230, 274]

->
v[12] = w[0] * f[12]
      = 149 + 230 + 274
      = 653



k = 3
n:                    0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,    12,    13,    14,    15,    16,    17,    18,    19,    20,
v[n]:                 1,     1,     1,     3,     5,     9,    17,    31,    57,   105,   193,   355,   653,  1201,  2209,  4063,  7473, 13745, 25281, 46499, 85525,

  10011
                     [1,     1,     1]
v[0]     +1          [1,     1,     1]
                            [1,     1,     3]
v[1]     *2          [1,     1,     1]
v[10]    *2                 [1,     1,     3]
v[100]   *2                               [3,     5,     9]
v[1000]  +1                                                          [31,    57,   105]
v[1001]  *2                                                                 [57,   105,   193]
v[10010] +1                                                                                                                             [13745, 25281, 46499]
v[10011]                                                                                                                                       [25281, 46499, 85525]

*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn n3c21() {
        let k = 3;
        let n = 21;
        let mut w = vec![1; k];
        for _ in 0..n {
            print!("{:5}, ", w[0]);
            let mut sum = 0;
            for i in 0..k {
                sum += w[i];
            }
            for i in 0..k - 1 {
                w[i] = w[i + 1];
            }
            w[k - 1] = sum;
        }
        println!("");
        for i in 0..n {
            print!("{:5}, ", i);
        }
        println!("");
        assert_eq!(0, 1);
    }
    /*
    #[test]
    fn it_works() {
        /*
        assert_eq!(
            next(2, vec![vec![1, 0], vec![0, 1]]),
            vec![vec![0, 1], vec[]]
        )
        let c = solution_kitamasa(1, 2);
        assert_eq!(c, 1);
        let c = solution_kitamasa(2, 2);
        assert_eq!(c, 1);
        let c = solution_kitamasa(3, 2);
        assert_eq!(c, 2);
        let c = solution_kitamasa(4, 2);
        assert_eq!(c, 3);
        let c = solution_kitamasa(5, 2);
        assert_eq!(c, 5);
        let c = solution_kitamasa(6, 2);
        assert_eq!(c, 8);

        let c = solution_kitamasa(10, 2);
        assert_eq!(c, 55);

        let c = solution_kitamasa(10, 3);
        assert_eq!(c, 105);
        */
    }
    */
}
