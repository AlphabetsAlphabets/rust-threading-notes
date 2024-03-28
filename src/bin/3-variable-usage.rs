use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];
    let mut num = 5;

    // `move` is required to transfer ownership so that the
    // thread owns the `number`.
    let t = thread::spawn(move || { // now `number` can only be used in here
        num += 1;
        println!("Num: {:#?}", num);
        println!("Numbers: {:#?}", numbers);
    });

    // `move` is required because the main thread MAY finish faster than `t`.
    // If `number` is borrowed via reference meaning without `move`. 
    // Once `main` ends, `number` is dropped and the reference becomes invalid.

    // `num` can still be used because an integer implements `Copy`.
    println!("{}", num);
    
    // Line below will result in an error as `numbers` does not implement `Copy`.
    // println!("{:#?}", numbers); 

    t.join().unwrap();
}
