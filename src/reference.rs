use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
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

#[test]
fn test_rc() {
    let city = City {
        name: "ChongQing".to_owned(),
        population: 1_200_200,
        city_history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_owned()),
    };
    println!("strong_count: {}", Rc::strong_count(&city.city_history));

    let cities = CityData {
        names: vec![city.name],
        histories: vec![city.city_history.clone()],
    };

    println!("Cq history is: {:?}", city.city_history);
    println!("cities: {:?}", cities);
    println!("strong_count: {}", Rc::strong_count(&city.city_history));

    let new_city = city.city_history.clone();
    println!("strong_count: {}", Rc::strong_count(&new_city));
}
