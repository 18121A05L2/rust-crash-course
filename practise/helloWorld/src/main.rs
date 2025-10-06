#![allow(unused)]

//  let y = 5; // this will throw an error
const NUM: i32 = 5;

use std::fs::File;
fn main() {
    println!("Hello, world!");
    let x = 5;
    // x = 10; this will be an error
    println!("The value of x is {}", x);
    // inline
    println!("The value of x is {x}");
    // positional arguments
    println!("The value of {0} + {0} = {1}", x, x + x);

    let x: i32 = x + 1; // shadowing

    let x: bool = true; // shadowing

    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;

    const STRING: &str = "Hello";

    let unsigned_max: u32 = u32::MAX;
    let signed_max: i32 = i32::MAX;
    println!("{}", unsigned_max);
    println!("{}", signed_max);

    let _a = "a";

    let k = 2.0;

    let _difference = k - MAX_POINTS as f64;

    // ---------------------------- Arrays and Tuples ---------------------------

    let arr = [1u32, 2, 3];
    let b = arr.len();

    println!("The value of arr[0] is {}", arr[1]);
    println!("length of the arr is {}", arr.len());

    let tuple = (1, 2, true, "hello");

    println!("The value of tuple.0 is {}", tuple.0);

    let zero_array = [0; 10];
    println!("{:?}", zero_array);

    // ----------------------------- Strings -----------------------------

    fn test_string(s: String) -> String {
        s
    }
    let m = test_string(String::from("Hello"));
    println!("{}", m);

    // You can easily convert from String → &str (borrowing):
    let s: String = String::from("hello");
    let _slice: &str = &s; // borrow as &str

    // But not the other way automatically (&str → String) — you must allocate a new String:
    let slice: &str = "hello";
    let _s: String = slice.to_string(); // or String::from(slice)

    // ------------------- Struct --------------------
    struct Car {
        make: String,
        cost: String,
    }

    let car1 = Car {
        make: String::from("Honda"),
        cost: String::from("Cheap"),
    };

    println!("{}", car1.make);

    struct TupleStruct(String, String); // tuple type structs

    let car2 = TupleStruct(String::from("Honda"), String::from("Cheap"));
    println!("{:?}", car2.0);

    let TupleStruct(x, y) = car2; // tuple destructuring
    println!("{}, {}", x, y);

    // ------------------------------ Enums --------------------------

    enum Directions {
        Up,
        Down,
        Left,
        Right,
    }
    // types enum
    enum DirectionsTyped {
        Up(String),
        Down(String),
        Left(String),
        Right(String),
        Car { make: String, cost: String },
    }

    let up = Directions::Up;
    let up2 = DirectionsTyped::Up(String::from("up"));
    // println!("{}", up);

    // --------------------------- if else and match(switch) expresstion ----------------

    let x = 10;
    if x > 5 {
        println!("x is greater than 5");
    }

    fn test_loop() {
        loop {
            println!("loop");
        }
    }
    // test_loop();

    let a: [i32; 6] = [1, 2, 3, 45, 5, 4];

    for i in a.iter() {
        println!("{}", i);
    }

    for item in 0..4 {
        println!("{}", item);
    }

    // -------------------------- Error handling --------------------------

    // panic!("error");
    // println!("After error thrown");

    let vec = vec![1, 2, 3];
    // println!("{}", vec[10]);

    let f = File::open("src/hello.txt");
    println!("{:?}", f);

    //----------------------- scope / clone ----------------------

    fn test_clone(x: String) {
        println!("{}", x);
    }

    fn test_func_borrowing(x: &String) {
        println!("{}", x);
    }

    fn test_scope() {
        let mut x = String::from("hello");
        test_clone(x.clone());
        test_func_borrowing(&x);
        test_func_borrowing(&mut x); // mutable using borrowing
        println!("{}", x);
    }
    test_scope();

    // ---------------------- dereferencing -----------------------
    let mut x = 5;
    let y = &mut x;

    *y += 1; // ✅ must dereference to modify
    println!("{}", *y); // ✅ must dereference to read as a value

    // -----------------------Module ------------------------------

    mod root {
        mod test_mod {
            pub fn test() {
                println!("test");
            }
        }
        mod tes_mod2 {
            use super::super::test_mod as fromRoot;
            use super::test_mod;
            pub fn test() {
                test_mod::test();
                fromRoot::test();
                println!("test");
            }
        }
    }

    // ----------------------- Others ----------------------------

    let mut s = String::from("hello");
    let s2 = &mut s;
    let s1 = &mut s;
    // s2.push_str(" world");

    // s2.push_str(" world");

    println!("{}", s1);
}

mod test_mod {
    pub fn test() {
        println!("test");
    }
}

mod tes_mod2 {
    use super::test_mod;
    pub fn test() {
        println!("test");
    }
}
