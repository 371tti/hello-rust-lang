// to run
// cargo run --example friendly_error

fn main() {
    let s = "hello".to_string();
    let s_sec = s; // move ownership
    println!("{}", s_sec);
    println!("{}", s); // ← error: borrow of moved value: `s`
}

// output

//  cargo run --example friendly_error
//    Compiling hello-rust-lang v0.1.0 (Z:\D\dev\lang_test\hello-rust-lang)
// error[E0382]: borrow of moved value: `s`
//  --> examples\friendly_error.rs:5:20
//   |
// 2 |     let s = "hello".to_string();
//   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
// 3 |     let s_sec = s; // move ownership
//   |                 - value moved here
// 4 |     println!("{}", s_sec);
// 5 |     println!("{}", s); // ← error: borrow of moved value: `s`
//   |                    ^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     let s_sec = s.clone(); // move ownership
//   |                  ++++++++

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `hello-rust-lang` (example "friendly_error") due to 1 previous error

// it very friendly error message. so i love rust.
