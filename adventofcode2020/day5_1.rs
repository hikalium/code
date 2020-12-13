use std::cmp;
use std::io::{self, Read};

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /*
    #[test]
    fn test_pid() {
        assert_eq!(is_valid_pid("000000001"), true);
        assert_eq!(is_valid_pid("0123456789"), false);
    }
    */
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let entries: Vec<&str> = buffer.split('\n').collect();
    let mut max_key = 0;
    for e in entries {
        if e.len() == 0 {
            continue;
        }
        let row_key = &e[0..7].replace("F", "0").replace("B", "1");
        let col_key = &e[7..10].replace("L", "0").replace("R", "1");
        let key_str = row_key.to_string() + col_key;
        let key = isize::from_str_radix(&key_str, 2).unwrap();

        println!("{} {} {} {} {}", e, row_key, col_key, key_str, key);
        max_key = cmp::max(max_key, key);
    }
    println!("{} is the max key", max_key);
    Ok(())
}
