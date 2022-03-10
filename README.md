# rustic-c
*An extraordinarily horrible idea.*

### If you want to know how this works, please [read this](HOW_ITS_MADE.md)

## What?
Is Rust too memory-safe and elegant for you?
Do you wish to return to something simpler?
Well, have I got the solution for you, with
the all new `c!` macro! Simply insert `c!` into your source
and then write C like it's 1999!\*

\* _Terms and conditions apply._

## How?
It's simple! Just use `c!` and write C!
You can even call back to Rust!
```rs
fn a_callback() {
    println!("hi from rust");
}

fn main() {
    unsafe {
        c! {
            void func() {
                'a_callback();
            }
        }
    }
}