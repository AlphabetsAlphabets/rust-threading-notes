use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // A scoped thread will borrow via reference.
    // Only use this when you are use the thread
    // will finish executing before the variables
    // borrowed are not dropped.
    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{}", n);
            }
        });
    });

    println!("Sum: {}", numbers.iter().sum::<usize>());
}
