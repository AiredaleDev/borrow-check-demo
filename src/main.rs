use std::rc::Rc;

fn first_example() {
    // Rust has two string types: `str` (the primitive) and `String`, which wraps `str`.
    // str is more akin to [char] (ptr to start + len). It is best thought of as a pointer.
    // String is a heap-allocated str (Box<str>). It is best thought of as a value on the heap.
    let s = String::from("Howdy, Sailor");
    let t = String::from("Did you statically check the scallywag?");

    // Notice how this function *consumes* its inputs.
    // By default (like all values), function arguments are immutable by default.
    // You can write a more readable/maybe performant version of this function (seen below), but
    // I don't want you to think that mutability causes movement.
    fn concat_strings(prefix: String, between: &str, suffix: String) -> String {
        // `chars` merely borrows its input, but this function still owns & cleans up `prefix` and `suffix`
        // because they go out-of-scope here.
        prefix
            .chars()
            .chain(between.chars())
            .chain(suffix.chars())
            .collect()
    }

    // This function does the same as the one above, but is written in a more imperative style.
    // It also will grow `prefix`'s buffer.
    fn other_concat_strings(mut prefix: String, between: &str, suffix: String) -> String {
        // Grows `prefix`'s pre-existing buffer to accommodate the contents of `suffix` and `between.`
        prefix.push_str(between);
        prefix.push_str(&suffix);
        prefix
    }

    // Observe the compiler error!
    // How can you change the signature of concat_strings to get this to compile?
    // You should listen to its suggestion regarding the function's signature for better performance.
    let new_s = concat_strings(s, ". ", t);
    println!("Mapped \"{s}\" and \"{t}\" into \"{new_s}\"");

    assert_eq!(new_s, other_concat_strings(s, ". ", t));
}

// `Debug` generates code to print this value.
#[derive(Debug)]
struct BunchaData {
    s: String,
    t: Vec<usize>,
    curr: usize,
}

impl BunchaData {
    fn with_empty_string(t: Vec<usize>) -> Self {
        Self {
            s: String::new(),
            t,
            curr: 0,
        }
    }
}

fn structs_automove_too() {
    let s = String::from("Ok");
    let t = vec![4; 10];

    // No (deep) copies occur in the construction of `clump.`
    let clump = BunchaData { s, t, curr: 0 };
    // Once again, we observe the same behavior.
    // This time, there is no function signature to change.
    // You either only access `s` through `clump` or you copy `s`.
    println!("I built {clump:?} using {s} and {t:?}");

    let mut t2 = vec![2, 3, 5, 7, 11];
    t2.push(13);
    // As previously established, t2 is not deep-copied for this function call.
    // It is now a member of other_clump.
    let other_clump = BunchaData::with_empty_string(t2);
    println!("Here's my other clump: {other_clump:?}");
}

fn auto_copy() {
    let x = 1;
    let y = 2;

    fn arith_man(x: i32, y: i32) -> i32 {
        // You likely expect this function to consume x and y based on the previous behavior...
        x * y + 10
    }

    // But you'll notice no compilation error occurred!
    let z = arith_man(x, y);
    // Integer values implement `Copy`
    println!("x = {x}, y = {y} -> arith_man(x,y) = {z}");
}

// Try removing `Copy` from the list of derives
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    const ONES: Self = Self {
        x: 1.,
        y: 1.,
        z: 1.,
    };
}

fn defining_your_own_copy() {
    fn vec_add(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        }
    }

    let v1 = Vec3 {
        x: 4.,
        y: 2.,
        z: 0.,
    };

    let v2 = vec_add(v1, v1);
    let v3 = vec_add(v1, Vec3::ONES);
    let v4 = vec_add(v1, v3);
    println!(
        "Look at all the vectors I copied: {:?}, {:?}, {:?}, {:?}",
        v1, v2, v3, v4
    );

    // With this trait bound, we can also use this syntax to construct an array with any number of copies of that value!
    // We have stated that copying the value is trivial, so it is okay.
    let vec_arr = [Vec3 {
        x: 69.,
        y: 42.,
        z: 0.,
    }; 10];

    println!("The vec_arr: {vec_arr:?}");
}

fn borrows_and_their_lifetimes() {
    // Every value in Rust has an associated lifetime.
    // For 90% of code, the language just infers the right one.

    // Observe how switching the order of declaration
    // between mx and rx causes a compilation error.
    // If all uses/creations of immutable borrows happen before a
    // mutable borrow's creation + uses, then immutable and mutable borrows are allowed
    // to coexist in the same scope. The promise that `rx` points to an unchanged `x` is not violated.
    // This is called "non-lexical-lifetimes." It will be replaced with an even more flexible system
    // called Polonius, which I don't have time to get into here.
    let mut x = 10;
    let rx = &x;
    let mx = &mut x;

    let mut y = 20;
    let ry = &y;
    let ry2 = &y;
    let my = &mut y;
    // Uncomment to cause compilation error.
    // let my2 = &mut y;

    use std::mem::swap;
    swap(mx, my);

    println!("Introducing the new values of x and y: x = {x}, y = {y}");
}

// Storing references in structs requires that you
// specify how long they live in relation to the struct.
// This is because the memory that the reference is pointing to is not a part of the struct itself,
// and can live arbitarily longer than the struct holding the pointer.
// This struct asserts that the data these two pointers point to must live for the same duration.
struct CantInfer<'a> {
    s: &'a str,
    t: &'a [usize],
}

// See Jon Gjegnset's stream for more information on this.
// https://youtu.be/rAl-9HwD858
struct TwoLifeTimes<'a, 'b> {
    slice: &'b mut &'a str,
}

fn main() {
    // To see the runtime behavior of any of the above,
    // simply call them right here!
}
