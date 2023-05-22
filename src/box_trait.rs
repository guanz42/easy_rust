#![allow(dead_code)]

use core::fmt;
use std::error::Error;

trait JustATrait {} // We will implement this on everything

#[derive(Debug)]
enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}
impl JustATrait for EnumOfNumbers {}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}

enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}

struct ArrayAndI8 {
    array: [i8; 1000], // This one will be very large
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}

#[test]
fn test_box() {
    use std::mem::size_of;

    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );
}

fn returns_just_a_trait() -> Box<dyn JustATrait> {
    let some_enum = EnumOfNumbers::I8(33);
    let s = Box::new(some_enum);
    // println!("{}", size_of::<Box<EnumOfNumbers>>());
    s
}

#[test]
fn test_returns_just_a_trait() {
    let s = returns_just_a_trait();
    println!("{:p}", s);
}

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

fn return_errors(intput: u8) -> Result<String, Box<dyn Error>> {
    match intput {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Look fine to me".to_owned()),
    }
}

#[test]
fn test_dyn_trait() {
    let vec_of_u8s = vec![0_u8, 1, 80];

    let _fourth = vec_of_u8s.get(3).unwrap_or_else(|| {
        if vec_of_u8s.get(0).is_some() {
            &vec_of_u8s[0]
        } else {
            &0
        }
    });

    let _all = vec_of_u8s.iter().all(|x| x > &2);

    let double_vec = vec_of_u8s
        .iter()
        .map(|number| number * 2)
        .collect::<Vec<u8>>();
    println!("{:?}", double_vec);

    for number in vec_of_u8s {
        match return_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}
