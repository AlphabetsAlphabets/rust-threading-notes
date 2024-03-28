use std::thread;

fn f() {
    println!("Hi from thread {:?}", thread::current().id());
}

fn main() {
    // Outputs will always be different. Sometimes you don't
    // even get outputs from the spawned threads. This is because
    // the main function finishes, the whole program terminates
    // taking out all the spawned threads in the process.
    thread::spawn(f);
    thread::spawn(f);

    println!("Hi from main thread.");
}
