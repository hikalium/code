const MOD: u64 = 1_000_000_007;

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

fn inner_product(lhs: &[u64], rhs: &[u64]) -> u64 {
    lhs.iter()
        .zip(rhs.iter())
        .fold(0, |s, v| (s + (v.0 * v.1) % MOD) % MOD)
        % MOD
}

fn f_next(f: &[u64]) -> Vec<u64> {
    // f[n+1] = [f[n][2], f[n][0] + f[n][2], f[n][1] + f[n][2]] ..... *next
    let mut nf = Vec::new();
    let last = *f.last().expect("f is empty");
    nf.push(last);
    for e in f.iter().take(f.len() - 1) {
        nf.push((last + e) % MOD);
    }
    nf
}

fn f_double(f: &[Vec<u64>]) -> Vec<u64> {
    // f[2*n] = (f[n] * f[n][0] + f[n+1] * f[n][1] + f[n+2] * f[n][2]) ..... *double
    let k = f.len();
    let mut nf = vec![0; k];
    for i in 0..k {
        for (t, nft) in nf.iter_mut().enumerate().take(k) {
            *nft += (f[i][t] * f[0][i]) % MOD;
            *nft %= MOD;
        }
    }
    nf
}

fn solution_kitamasa(k: usize, n: usize) -> u64 {
    if n < k {
        return 1;
    }
    let target_idx = n - 1;
    let num_of_bits = {
        let mut i = 0;
        loop {
            if target_idx >> i == 0 {
                break i;
            }
            i += 1;
        }
    };
    // Setup f[1]
    let mut f = vec![0; k];
    f[1] = 1;

    for i in (0..num_of_bits - 1).rev() {
        // f[n] -> f[2*n]
        let mut fs = vec![f];
        for _ in 1..k {
            fs.push(f_next(fs.last().expect("fs is empty")));
        }
        f = f_double(&fs);

        if (target_idx >> i) & 1 == 1 {
            // f[n] -> f[n+1]
            f = f_next(&f);
        }
    }
    inner_product(&vec![1; k], &f) % MOD
}

fn main() -> std::io::Result<()> {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let ev: Vec<&str> = rows[0].split(' ').collect();
    let k = ev[0].parse::<usize>().unwrap();
    let n = ev[1].parse::<usize>().unwrap();

    println!("{}", solution_kitamasa(k, n));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_inner_product() {
        assert_eq!(inner_product(&vec![1, 2, 3], &vec![4, 5, 6]), 4 + 10 + 18);
    }

    fn check_next_and_double(k: usize, n: usize) {
        let mut w = vec![1; k];
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(w[0]);
            let mut sum = 0;
            for i in 0..k {
                sum += w[i];
            }
            for i in 0..k - 1 {
                w[i] = w[i + 1];
            }
            w[k - 1] = sum;
        }

        let mut f = Vec::new();
        for i in 0..k {
            let mut fe = vec![0; k];
            fe[i] = 1;
            f.push(fe);
        }

        println!("testing f_next()...");
        let mut fe = vec![1; k];
        for i in k..n {
            println!("f[{}] = {:?}", i, fe);
            f.push(fe.clone());
            let e = inner_product(&vec![1; k], &fe);
            assert_eq!(e, v[i]);
            fe = f_next(&fe);
        }
        println!("f_next is OK");

        println!("testing f_double()...");
        for i in 0..n / 2 {
            let input_fslice = &f[i..i + k];
            let expected = &f[i * 2];
            println!(
                "f[{}..{}] = {:?} -> f[{}] = {:?}",
                i,
                i + k,
                &input_fslice,
                i * 2,
                &expected
            );
            let e = f_double(input_fslice);
            assert_eq!(&e, expected);
        }
    }

    #[test]
    fn k3n21() {
        check_next_and_double(3, 21);
    }

    #[test]
    fn k2n21() {
        check_next_and_double(2, 21);
    }

    #[test]
    fn k5n21() {
        check_next_and_double(5, 21);
    }

    #[test]
    fn samples() {
        assert_eq!(solution_kitamasa(3, 10), 105);
        assert_eq!(solution_kitamasa(2, 10), 55);
        assert_eq!(solution_kitamasa(2, 100), 687995182);
        assert_eq!(solution_kitamasa(2, 513), 169404872);
        assert_eq!(solution_kitamasa(2, 500), 550656477);
        assert_eq!(solution_kitamasa(2, 1000), 517691607);
    }
}
