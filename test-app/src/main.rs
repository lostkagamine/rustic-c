//use runtime_c::{compile_c, do_horrible_crimes};
use c_macro::c;

/*
fn say_hi() {
    println!("oh no");
}

fn main() {
    unsafe {
        let (ptr, size) = compile_c(&format!("
int one() {{
    ((void(*)()){:#x})();
    return 5;
}}", (say_hi as *const ()) as u64));
        let x = do_horrible_crimes::<i32>(ptr, size);
        println!("{x}");
    }
    println!("Hello, world!");
}
*/

fn main() {
    unsafe {
        c! {
            int x() {
                return 5;
            }
        }
    }
}