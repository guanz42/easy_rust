pub mod box_trait;
pub mod channel;
pub mod closure;
pub mod cow;
pub mod deref;
pub mod iter;
pub mod r#macro;
pub mod reference;
mod result;
pub mod thread;

use core::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::{
    cell::{Cell, RefCell},
    fmt::Display,
};

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

#[allow(dead_code)]
#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

#[test]
fn cell() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    // 10 years later, super_phone_3000 is not on sale anymore
    super_phone_3000.on_sale.set(false);
    println!("{:?}", super_phone_3000)
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

#[test]
fn user() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_owned(),
        active: RefCell::new(true),
    };

    println!("{:?}", user_1.active);

    let date = 2100;
    user_1
        .active
        .replace_with(|_| if date > 2000 { false } else { true });
}

fn print_number<T: Debug>(number: T) {
    // <T: Debug> is the important part
    println!("Here is your number: {:?}", number);
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn compare_and_display_v2<T, U>(statement: T, num_1: U, num_2: U)
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

fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).unwrap_or(&42);
    *fourth
}

#[test]
fn expect() {
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
    println!("fourth= {}", fourth);

    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec
        .into_iter() // "iterate" over the items (iterate = work with each item inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect::<Vec<i32>>(); // put them in a new Vec<i32>

    println!("{:?}", new_vec);
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec
        .into_iter() // "iterate" over the items (iterate = work with each item inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect::<Vec<i32>>(); // put them in a new Vec<i32>

    println!("{:?}", new_vec);

    let num_vec = vec![10, 9, 8];
    num_vec
        .iter()
        .enumerate()
        .for_each(|(i, v)| println!("{} = {}", i, v));
}

fn return_string() -> &'static str {
    let _s = String::from("hello world");
    "this is a string"
}

#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

#[test]
fn lifetime() {
    let a = return_string();
    println!("string a: {}", a);

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    {
        let city = City {
            name: &city_names[0],
            date_founded: 123,
        };
        println!("city: {:?}", city);
    }
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
