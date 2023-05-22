#![allow(dead_code)]

pub mod arc;
pub mod box_trait;
pub mod cell;
pub mod channel;
pub mod closure;
pub mod cow;
pub mod deref;
pub mod iter;
pub mod lifetime;
pub mod r#macro;
pub mod reference;
mod result;
pub mod thread;
pub mod vec;

use std::fmt::Display;

fn main() {
    let mut v = Vec::new();
    v.push(String::from("hello"));
    v.push(String::from("world"));
    println!("vec: {:?}, capacity= {}", v, v.capacity());

    let vec1 = vec!["a", "b", "c"];
    let vec2 = &vec1[..];
    println!("vec2: {:?}, capacity = {}", vec2, vec1.capacity());

    let my_vec2: Vec<_> = [9, 0, 10].into();
    println!("my_vec2: {:?}, capacity = {}", my_vec2, my_vec2.capacity());
}

fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}
