use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let rows: Vec<&str> = buffer.split('\n').collect();
    let mut trees = 0;
    let mut x = 0;
    for (i, row) in rows.iter().enumerate() {
        if row.len() == 0 {
            break;
        }
        let is_tree = row.chars().nth(x).unwrap() == '#';
        println!("{} {} {}", i, row, is_tree);
        if is_tree {
            trees = trees + 1;
        }
        x = (x + 3) % row.len()
    }
    println!("{} trees found.", trees);
    /*
    let mut valid_count = 0;
    for e in v {
        let ev: Vec<&str> = e.split(':').collect();
        if ev.len() != 2 {
            assert!(e.len() == 0);
            continue;
        }
        let password = ev[1];
        let ev: Vec<&str> = ev[0].split(' ').collect();
        let char_to_be_included = ev[1].chars().nth(0).unwrap();
        let ev: Vec<&str> = ev[0].split('-').collect();
        let left = ev[0].parse::<usize>().unwrap();
        let right = ev[1].parse::<usize>().unwrap();
        let mut counter_in = 0;
        let mut counter_out = 0;
        for (i, c) in password.chars().enumerate() {
            if c != char_to_be_included {
                continue;
            }
            if i == left || i == right {
                counter_in = counter_in + 1;
            } else {
                counter_out = counter_out + 1;
            }
        }
        let is_valid = counter_in == 1;
        println!(
            "{}\t-\t{},\t'{}',\t\"{}\", {}, {}, {}",
            left, right, char_to_be_included, password, counter_in, counter_out, is_valid
        );
        if is_valid {
            valid_count = valid_count + 1;
        }
    }
    println!("{} passwords are valid", valid_count);
    */
    Ok(())
}
