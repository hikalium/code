use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", buffer);
    let v: Vec<&str> = buffer.split('\n').collect();
    let mut valid_count = 0;
    for e in v {
        let ev: Vec<&str> = e.split(':').collect();
        if ev.len() != 2 {
            continue;
        }
        let password = ev[1];
        let ev: Vec<&str> = ev[0].split(' ').collect();
        let char_to_be_included = ev[1].chars().nth(0).unwrap();
        let ev: Vec<&str> = ev[0].split('-').collect();
        let left = ev[0].parse::<i32>().unwrap();
        let right = ev[1].parse::<i32>().unwrap();
        let mut counter = 0;
        for c in password.chars() {
            if c == char_to_be_included {
                counter = counter + 1;
            }
        }
        let is_valid = left <= counter && counter <= right;
        println!(
            "{} - {}, '{}', \"{}\", {}, {}",
            left, right, char_to_be_included, password, counter, is_valid
        );
        if is_valid {
            valid_count = valid_count + 1;
        }
    }
    println!("{} passwords are valid", valid_count);
    Ok(())
}
