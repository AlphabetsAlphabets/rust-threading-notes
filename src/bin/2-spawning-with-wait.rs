use std::thread;

fn f() {
    println!("Hi from thread {:?}", thread::current().id());
}

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    // Join makes sure the threads are finished before
    // the main function exists. However the order in which
    // each thread finishes executing is undefined. This
    // includes the main thread itself.
    t1.join().unwrap();
    t2.join().unwrap();

    println!("Hi from main thread again.");
}
