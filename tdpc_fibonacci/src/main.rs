#![feature(map_first_last)]
use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

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

fn solution_kitamasa(n: usize, k: usize) -> u64 {
    let mut tmp_c = HashMap::new();
    // Calcurates a sequence of coefficients that gives
    // a_(x+i) = c[0]*a_(x+0) + c[1]*a_(x+1) + ... + c[k-1]a_(x+k-1)
    let c = calc_coefficients(n - 1, k, &mut tmp_c);
    println!("{:?}", c);
    c.iter().fold(0, |sum, v| sum + *v as u64) % MOD
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let ev: Vec<&str> = rows[0].split(' ').collect();
    let k = ev[0].parse::<usize>().unwrap();
    let n = ev[1].parse::<usize>().unwrap();

    //println!("{}", solution_companion_mat(n, k));
    println!("{}", solution_companion_mat2(n, k));
    println!("{}", solution_kitamasa(n, k));

    //println!("{}", solution_naive(n, k));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
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
    }
}
