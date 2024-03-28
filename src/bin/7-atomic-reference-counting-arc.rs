use std::sync::Arc;
use std::thread;

fn main() {
    let original = [1, 2, 3];

    // Ensures that `a` is dropped only 
    // when it is no longer is use by ANY thread.
    let a = Arc::new(original);
    let b = a.clone();

    let t1 = thread::spawn(move || {
        println!("{:#?}", a);
    });

    let t2 = thread::spawn(move || {
        println!("{:#?}", b);
    });

    t1.join().unwrap();
    t2.join().unwrap(); // `a` is dropped here but because of `Arc` it as
                        // `t2` will finish after `t1` and `t2` is the final
                        // thread that uses `a`.

    // `original` is a `Copy` type so this works fine.
    println!("{:#?}", original);

    let original = vec![1, 2, 3];
    let a = Arc::new(original);

    let t1 = thread::spawn(move || {
        println!("{:#?}", a);
    });

    t1.join().unwrap(); // `a` is dropped here.
    
    // Line below results in an error because `original` is a `Vec`
    // and `Vec`s are not `Copy` types.
    // println!("{:#?}", original);

    // Shadowing is preferred when using `Arc`s because they point
    // to the same data, just in a different variable name. The way to
    // shadow without removing the original `Arc` is by creating a new scope.

    let a = Arc::new(vec![4, 5, 6]);
    thread::spawn({
        let a = a.clone();
        move || {
            println!("{:#?}", a);
        }
    }).join().unwrap();

    println!("{:#?}", a);
} 
