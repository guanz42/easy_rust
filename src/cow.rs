#![allow(dead_code)]
use std::borrow::Cow;

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}

#[test]
fn test_cow() {
    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(msg) => println!("{} went in, [Borrowed]:{}", number, msg),
            Cow::Owned(msg) => println!("{} went in, [Owned]:{}", number, msg),
        }
    }
}
