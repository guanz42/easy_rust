#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread::spawn};

    #[test]
    fn test_channel() {
        let (sender, receiver) = channel();
        let sender_clone = sender.clone();

        let mut handle_vec = vec![];
        let mut results = vec![];

        handle_vec.push(spawn(move || {
            sender.send("Hello Rust").unwrap();
        }));
        handle_vec.push(spawn(move || {
            sender_clone.send("Learn channel").unwrap();
        }));

        for _ in handle_vec {
            let data = receiver.recv().unwrap();
            results.push(data);
        }

        println!("{:?}", results)
    }

    #[test]
    fn test_channel_ext() {
        let (sender, receiver) = channel();
        let hugevec = vec![0; 1_000_000];
        let mut handle_vec = vec![];
        let mut newvec = vec![];

        for i in 0..10 {
            let sender_clone = sender.clone();
            let mut work = Vec::with_capacity(hugevec.len() / 10);
            work.extend(&hugevec[i * 100_000..(i + 1) * 100_000]);

            let handle = spawn(move || {
                for num in work.iter_mut() {
                    *num += 1;
                }
                sender_clone.send(work).unwrap();
            });
            handle_vec.push(handle);
        }

        for handle in handle_vec {
            handle.join().unwrap();
        }

        while let Ok(results) = receiver.try_recv() {
            newvec.push(results);
        }

        let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>();
        println!(
            "{:?}, {:?}, total length: {}", // Let's print out some numbers to make sure they are all 1
            &newvec[0..10],
            &newvec[newvec.len() - 10..newvec.len()],
            newvec.len() // And show that the length is 1_000_000 items
        );

        for number in newvec {
            // And let's tell Rust that it can panic if even one number is not 1
            if number != 1 {
                panic!();
            }
        }
    }
}
