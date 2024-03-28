use std::cell::Cell;

fn not_copy_type(vec: &Cell<Vec<i32>>) {
    // non `Copy` types will need to use `take` which will
    // grab the value out of the `Cell` and replace it with
    // the `default::Default()` of what you just took.
    //
    // In the case of the vec, an empty vec is left.
    let mut new_vec = vec.take();
    // Mutating an immutable variable is allowed by outright replacing
    // the original value with a new one. So, it's more replacing than
    // mutating. This replacing is called "interior mutability".
    new_vec.push(20);
    vec.set(new_vec);
}

fn copy_type(num: Cell<u8>) {
    // `Copy` types get to use `get` which will simply copy the value out of `Cell`.
    // This is called creating a "shared reference" (&T). Mutating a value this
    // way is allowing mutations through shared reference. To make mutations with
    // an exclusive reference (&mut T) look at refcell.
    let mut new_num = num.get();
    new_num += 1;
    num.set(new_num)
}

fn main() {
    // When using a Cell you can use either a `Copy` or non-`Copy`.
    // In both cases, when modifying a value in a cell. You need to
    // take it out, add the new stuff then put it back in.
    let a = Cell::new(vec![1, 2, 3]);
    // Note that the value passed into `a` is immutable. Applies to `b`
    // as well.
    let b = Cell::new(20);

    not_copy_type(&a);
    copy_type(b);

    // Can't use `b` here because it owned by `bar`.
    // println!("{}", b.get());
}
