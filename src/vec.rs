#![allow(dead_code)]

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

    let num_vec = vec![10, 9, 8];
    num_vec
        .iter()
        .enumerate()
        .for_each(|(i, v)| println!("{} = {}", i, v));
}
