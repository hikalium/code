use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer = buffer.replace(" ", "\n");
    let entries: Vec<&str> = buffer.split('\n').collect();
    let mut passport = HashMap::new();
    let mut valid_count = 0;
    for e in entries {
        let kv: Vec<&str> = e.split(':').collect();
        if kv.len() != 2 {
            if passport.contains_key("byr")
                && passport.contains_key("iyr")
                && passport.contains_key("eyr")
                && passport.contains_key("hgt")
                && passport.contains_key("hcl")
                && passport.contains_key("ecl")
                && passport.contains_key("pid")
            {
                valid_count = valid_count + 1;
            }
            passport = HashMap::new();
            println!("--");
            continue;
        };
        passport.insert(kv[0], kv[1]);
        println!("{}", e);
    }
    println!("{} valid passports found", valid_count);
    Ok(())
}
