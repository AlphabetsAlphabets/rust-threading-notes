use std::rc::Rc;

// Reference counting is a system that ensures that when data
// is sharred it is properly dropped and deallocated. 
fn main() {
    // This creates a shared variable across threads.
    let original = [1, 2, 3];
    let a = Rc::new(original);
    // Rc keeps track of how many threads own a REFERENCE to `original`
    // once the owners drops to 0, then `original` will be dropped.

    let b = a.clone();
    // `as_ptr` gets the pointer to the raw data.
    println!("a == b? {}", b.as_ptr() == a.as_ptr());

    // When comparing the two the data is the same. Meaning data is
    // not cloned. Instead it's a reference to the same piece of data.
    // A vector is not a Copy type, so if you did `let b = original`. 
    // `original` becomes invalid as `b` is now the new owner.
    
    // Rc is suitable for single threaded applications but
    // Arc is suitable for multi-threaded applications.
}
