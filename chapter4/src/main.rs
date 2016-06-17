// Currently at:
// https://doc.rust-lang.org/book/if.html

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

/// Standard if, else if, else in rust
/// Also showcasing branched variables
fn branching() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }
    let y = if x == 5 { 10 } else { 15 }; // y: i32
}

/// Infinite loop until breakage in rust
fn infinite() {
    loop {
        println!("Print this infinite times!");
    }
}

/// While loop correct when unsure how many times a loop is needed
fn while_loop() {
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

/// For loop for particular number of times. Expression based. Expression merely need to be able to
/// use IntoIterator
fn for_loop() {
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
}


/// Enumeration for when you need to keep track of how many times you've already enumerated
fn enumeration() {
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}

/// Breaking and continuing in loops.
/// Rust allows the regular break to get out of loops. It does however also have continue
/// when you merely want to move onwards to next iteration
fn breaking_and_continuing() {
    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }
}

/// Additionally, rust allows labeling of loops such that on can easily break out to,
/// or continue at a specific loop in cases of several nested loops
fn nested_loop_breaking_and_continuing() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }
}

/// Vectors can be initialized with repeating values or with complete declarations
fn vector_initialization() {
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    let v = vec![0; 10]; // ten zeroes
}

/// Per usual, vector indexing starts at 0
/// Rather unusually, refering an index takes a type usize instead of int
/// let i: usize = 0;
/// let j: i32 = 0;
/// Inferring an index with i will work, inferring it with j will not
fn vector_access() { 
    let v = vec![1, 2, 3, 4, 5];

    println!("The third element of v is {}", v[2]);
}

/// If we want to access an array with unsafe indices, we can use get
/// to catch the panic that will otherwise be called
fn out_of_bounds_access() {
    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }
}

/// Vector iteration can be done multiple ways, yielding multiple results
/// The version taking ownership of the array yields it unusable and thereby uncompilable if this
/// is attempted.
/// Instead the references version should be used if it is to remain usable.
/// If changes are to be made throughout iteration the mutable version should be used.
fn vector_iteration() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
