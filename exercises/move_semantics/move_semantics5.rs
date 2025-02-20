// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    // unsafe{
    //     let y = &mut x as *mut i32;
    //     let z = &mut x as *mut i32;
    
    // *y += 100;
    // *z += 1000;
    // }
    {
        let y = &mut x;
        *y += 100;
    }  // y 的作用域在这里结束
    {
        let z = &mut x;
        *z += 1000;
    }
    assert_eq!(x, 1200);
}