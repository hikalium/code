use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let ev: Vec<&str> = rows[0].split(' ').collect();
    let k = ev[0].parse::<usize>().unwrap();
    let n = ev[1].parse::<usize>().unwrap();

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
    println!("{}", kv[(n - 1) % k]);

    Ok(())
}
