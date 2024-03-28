use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let sum = numbers.iter().sum::<usize>();
        let len = numbers.len();

        // Returns are handled like in functions.
        sum / len
    });

    // calling `join` on a thread will return a `Result<T>`.
    if let Ok(avg) = t.join() {
        println!("Average: {}", avg);
    } else {
        println!("Error.");
    }
}
