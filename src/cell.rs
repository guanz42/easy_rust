#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::fmt::Debug;

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
