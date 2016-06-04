/// rustdoc can be used to compile HTML documentation
/// using the doc comments
fn main() {
    println!("x is: {}", return_number (32));
}

/// Doc commenting
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }

fn add_one(x: i32) -> i32 {
    x + 1
}

fn types() {
    //slices
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in a
    let middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3
    let x: (i32, &str) = (1, "hello");
}

fn deconstruction() {
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
}

fn tuples() {
    (0,); // single-element tuple
    (0); // zero in parentheses
    let tuple = (1, 2, 3);

    let x = tuple.0;//tuple referencing uses . rather than []
    let y = tuple.1;
    let z = tuple.2;
}

fn function_types() {
    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;//rebinding with no type inference
}
fn return_number(x: i32) -> i32 {
    x + 1
}
