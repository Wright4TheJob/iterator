use censor::*;
use std::env;

fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

fn pad_to_length(base: String, digits: usize, pad_with: char) -> String {
    if base.len() >= digits {
        return base
    }
    let quantity_needed = digits - base.len();
    let mut prepended = String::new();
    for _ in 0..quantity_needed {
        prepended.push(pad_with);
    };
    format!("{}{}", prepended, base)
}

#[test]
fn test_pad() {
    assert_eq!(pad_to_length("ABC".to_string(), 5, "0".chars().nth(0).unwrap()), "00ABC".to_string());
}

fn from_int(value: usize, valid_chars: &str) -> String {
    match value < valid_chars.len() {
        true => valid_chars.chars()
                    .nth(value)
                    .expect("Index requested is out of bounds")
                    .to_string(),
        false => {
            let (quot, rem) = div_rem(value, valid_chars.len());
            format!("{}{}", 
                from_int(quot, valid_chars), 
                from_int(rem, valid_chars))
        }
    }
}

#[test]
fn test_from_int_1() {
    let valid_chars = "ABC";
    assert_eq!(from_int(0, valid_chars), "A");
    assert_eq!(from_int(2, valid_chars), "C");
}

#[test]
fn test_from_int_2() {
    let valid_chars = "ABC";
    assert_eq!(from_int(0, valid_chars), "A");
    assert_eq!(from_int(2, valid_chars), "C");
    assert_eq!(from_int(3, valid_chars), "BA");
}


fn to_int(s: String, valid_chars: &str) -> usize {
    if s.len() == 0 {
        return 0
    }
    let first_char = s.chars().next().expect("String had no length in to_int()");
    let power = s.len() - 1;
    let base = valid_chars.len();
    let char_index = valid_chars.chars().position(|c| c == first_char).expect("Character not found in valid_chars");
    let converted_value = char_index * base.pow(power.try_into().unwrap());
    
    let final_value = converted_value + to_int(s[1..].to_string(), valid_chars);
    return final_value
}

#[test]
fn test_to_int_1() {
    let valid_chars = "ABC";
    assert_eq!(to_int("AAA".to_string(), valid_chars), 0);
    assert_eq!(to_int("AAC".to_string(), valid_chars), 2);
    assert_eq!(to_int("ABA".to_string(), valid_chars), 3);
}

fn next(last: String, valid_chars: &str, digits: usize) -> String {
    let censor = Censor::Standard + Censor::Sex + Censor::Zealous;
    let next_val = from_int(to_int(last, valid_chars) + 1, valid_chars);
    let next_val = pad_to_length(next_val, digits, valid_chars.chars().nth(0).unwrap());
    match censor.check(&next_val) {
        true => next(next_val, valid_chars, digits),
        false => next_val
    }
}

fn is_valid_char(ch: char, valid_chars: &str) -> bool {
    match valid_chars.find(ch) {
        Some(_) => true,
        None => false,
    }
}

fn is_valid_id(id: &str, valid_chars: &str) -> bool {
    let mut is_valid = true;
    for ch in id.chars() {
        is_valid = is_valid && is_valid_char(ch, valid_chars);
    };
    is_valid
}

fn main() {
    let valid_chars = "ABCDEFGHJKLMNOPQRSTUVWXYZ";
    // let mut i = 1;
    // let mut last_id = String::from("AAAA");
    // println!("{}",last_id);
    // while i < 100 {
    //     last_id = next(last_id.clone(), valid_chars, 4);
    //     println!("{}",last_id);
    //     i += 1;
    // }

    let mut num_to_print = 1;
    if let Some(arg2) = env::args().nth(2) {
        if let Ok(num) = arg2.trim().parse() {
            num_to_print = num;
        };
    }

    let mut i = 0;
    if let Some(arg1) = env::args().nth(1) {
        match is_valid_id(&arg1, valid_chars) {
            true => {
                while i < num_to_print {
                    println!("{}", next(arg1.clone(), valid_chars, arg1.len()));
                    i = i + 1;
                }
            },
            false => println!("ID contains illegal characters.")
        }
    }

}
