#![allow(unused)]

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
