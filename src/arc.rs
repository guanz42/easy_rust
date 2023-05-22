#[test]
fn test_arc() {
    use std::sync::{Arc, Mutex};
    use std::thread::spawn;

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
