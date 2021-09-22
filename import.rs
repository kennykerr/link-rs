#[derive(PartialEq)]
#[repr(C)]
struct complex {
    a: i32,
    b: i32,
}

#[derive(PartialEq)]
#[repr(C)]
struct more_complex {
    a: i64,
    b: i64,
    c: i64,
}

#[link(name = "export")]
extern "system" {
    fn return_simple() -> i64;
    fn return_complex() -> complex;
    fn return_more_complex() -> more_complex;
}

fn main() {
    unsafe {
        assert!(return_simple() == 123);
        assert!(return_complex() == complex { a: 123, b: 456 });
        assert!(return_more_complex() == more_complex { a: 123, b: 456, c: 7890 });
        println!("ok");
    }
}
