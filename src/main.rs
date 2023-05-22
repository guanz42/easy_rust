#![allow(dead_code)]

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
use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    // let url = "https://www.rust-lang.org/";
    // let output = "rust.md";

    // println!("fetching url: {}", url);
    // let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    // println!("converting html to markdown...");
    // let md = html2md::parse_html(&body);

    // fs::write(output, md.as_bytes()).unwrap();
    // println!("convert markdown has been saved to {}", output);

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

#[test]
fn test_arc() {
    let num = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];

    for _ in 0..2 {
        let num_clone = Arc::clone(&num);

        let handle = spawn(move || {
            for _ in 0..10 {
                *num_clone.lock().unwrap() += 1;
            }
        });

        handle_vec.push(handle);
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("{:?}", num)
}

#[test]
fn test_ref() {
    let mut number = 8;

    let number_change = &mut number;
    *number_change += 10;

    let number_ref = &number;
    println!("mut num_ref: {}", *number_ref);

    // let mut number = 10;
    // let number_ref = &number;
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);
}
