use std::io::{self, Read};

fn check_slope(rows: &Vec<&str>, dx: usize, dy: usize) -> io::Result<u64> {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < rows.len() {
        let row = rows[y];
        if row.len() == 0 {
            break;
        }
        let is_tree = row.chars().nth(x).unwrap() == '#';
        if is_tree {
            trees = trees + 1;
        }
        x = (x + dx) % row.len();
        y = y + dy;
    }
    Ok(trees)
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let diffs = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut ans = 1;
    for d in diffs {
        let trees = check_slope(&rows, d[0], d[1])?;
        println!("{} trees found.", trees);
        ans = ans * trees;
    }
    println!("Answer is {}", ans);
    Ok(())
}
