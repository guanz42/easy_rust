use core::time;

#[test]
fn thread() {
    // let mut my_string = String::from("I will go into the closure");
    // let mut my_closure = || {
    //     my_string.push_str(" now");
    //     println!("{}", my_string);
    // };
    // my_closure();
    // my_closure();

    for idx in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("hello world, {}", idx);
            std::thread::sleep(time::Duration::from_secs(1));
        });
        // handle.join();
    }
}
