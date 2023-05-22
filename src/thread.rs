#[test]
fn thread() {
    use core::time;

    for idx in 0..10 {
        let _handle = std::thread::spawn(move || {
            println!("hello world, {}", idx);
            std::thread::sleep(time::Duration::from_secs(1));
        });
        // handle.join();
    }
}
