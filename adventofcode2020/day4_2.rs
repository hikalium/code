use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, Read};

use regex::Regex;

lazy_static! {
    static ref RE_4D_NUM: Regex = Regex::new(r"^\d{4}$").unwrap();
    static ref RE_HGT: Regex = Regex::new(r"^\d+(cm|in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn is_valid_byr(s: &str) -> bool {
    if !RE_4D_NUM.is_match(s) {
        return false;
    }
    let v = s.parse::<i64>().unwrap();
    1920 <= v && v <= 2002
}
fn is_valid_iyr(s: &str) -> bool {
    if !RE_4D_NUM.is_match(s) {
        return false;
    }
    let v = s.parse::<i64>().unwrap();
    2010 <= v && v <= 2020
}
fn is_valid_eyr(s: &str) -> bool {
    if !RE_4D_NUM.is_match(s) {
        return false;
    }
    let v = s.parse::<i64>().unwrap();
    2020 <= v && v <= 2030
}
fn is_valid_hgt(s: &str) -> bool {
    if !RE_HGT.is_match(s) {
        return false;
    }
    if s.ends_with("cm") {
        let vs = &s[..s.len() - 2];
        let v = vs.parse::<i64>().unwrap();
        return 150 <= v && v <= 193;
    }
    if s.ends_with("in") {
        let vs = &s[..s.len() - 2];
        let v = vs.parse::<i64>().unwrap();
        return 59 <= v && v <= 76;
    }
    panic!();
}
fn is_valid_hcl(s: &str) -> bool {
    RE_HCL.is_match(s)
}
fn is_valid_ecl(s: &str) -> bool {
    RE_ECL.is_match(s)
}
fn is_valid_pid(s: &str) -> bool {
    RE_PID.is_match(s)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(is_valid_byr("true"), false);
        assert_eq!(is_valid_byr("1919"), false);
        assert_eq!(is_valid_byr("1920"), true);
        assert_eq!(is_valid_byr("2002"), true);
        assert_eq!(is_valid_byr("2003"), false);
    }
    #[test]
    fn test_iyr() {
        assert_eq!(is_valid_iyr("true"), false);
        assert_eq!(is_valid_iyr("2009"), false);
        assert_eq!(is_valid_iyr("2010"), true);
        assert_eq!(is_valid_iyr("2020"), true);
        assert_eq!(is_valid_iyr("2021"), false);
    }
    #[test]
    fn test_eyr() {
        assert_eq!(is_valid_eyr("true"), false);
        assert_eq!(is_valid_eyr("2019"), false);
        assert_eq!(is_valid_eyr("2020"), true);
        assert_eq!(is_valid_eyr("2030"), true);
        assert_eq!(is_valid_eyr("2031"), false);
    }
    #[test]
    fn test_hgt() {
        assert_eq!(is_valid_hgt("60in"), true);
        assert_eq!(is_valid_hgt("190cm"), true);
        assert_eq!(is_valid_hgt("190in"), false);
        assert_eq!(is_valid_hgt("190"), false);
    }
    #[test]
    fn test_hcl() {
        assert_eq!(is_valid_hcl("#123abc"), true);
        assert_eq!(is_valid_hcl("#123abz"), false);
        assert_eq!(is_valid_hcl("123abc"), false);
    }
    #[test]
    fn test_ecl() {
        assert_eq!(is_valid_ecl("brn"), true);
        assert_eq!(is_valid_ecl("wat"), false);
    }
    #[test]
    fn test_pid() {
        assert_eq!(is_valid_pid("000000001"), true);
        assert_eq!(is_valid_pid("0123456789"), false);
    }
}

fn main() -> io::Result<()> {
    assert!(RE_4D_NUM.is_match("2014"));
    assert!(!RE_4D_NUM.is_match("20145"));
    assert!(!RE_4D_NUM.is_match("201"));
    assert!(RE_HGT.is_match("123cm"));
    assert!(RE_HGT.is_match("45in"));
    assert!(!RE_HGT.is_match("123"));
    assert!(!RE_HGT.is_match("45"));
    assert!(RE_HCL.is_match("#123abc"));
    assert!(!RE_HCL.is_match("#123abz"));
    assert!(!RE_HCL.is_match("123abc"));
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
        let k = kv[0];
        let v = kv[1];
        if match k {
            "byr" => is_valid_byr(v),
            "iyr" => is_valid_iyr(v),
            "eyr" => is_valid_eyr(v),
            "hgt" => is_valid_hgt(v),
            "hcl" => is_valid_hcl(v),
            "ecl" => is_valid_ecl(v),
            "pid" => is_valid_pid(v),
            "cid" => true,
            _ => {
                panic!("Unexpected key {}", k);
            }
        } {
            passport.insert(k, v);
        }
        println!("{}", e);
    }
    println!("{} valid passports found", valid_count);
    Ok(())
}
