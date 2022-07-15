extern crate libc;

use libc::{c_char, c_double, c_int, c_uint, size_t};

mod meshing;
mod c_meshing;
// use cxx::CxxVector;

// #[cxx::bridge]
// mod ffi {
//     extern "Rust" {
//         fn test_vector(vec:CxxVector<f64>);
//     }
// }
// fn test_vector(vec:CxxVector<f64>)  {
//     let vv = vec![1,2,3];
// let res = CxxVector::from(vv);
// }

// test libc output
#[no_mangle]
pub extern "C" fn addition(a: c_int, b: c_int) -> c_int {
    println!("This is RUST！！");
    return a + b;
}

#[no_mangle]
pub extern "C" fn print(a: c_int, b: c_int) -> c_int {
    println!("这里是RUST！！");
    let s = a + b;
    // let s: CString = CString::new("feaf");
    return s;
}

#[no_mangle]
pub extern "C" fn sum_of_even(ptr: *const c_int, len: size_t) -> c_int {
    let slice = unsafe {
        assert!(!ptr.is_null());
        std::slice::from_raw_parts(ptr, len as usize)
    };

    let sum = slice
        .iter()
        .filter(|&&num| num % 1 == 0)
        .fold(0, |sum, &num| sum + num);
    sum as c_int
}
#[no_mangle]
pub unsafe extern "C" fn print_and_2double(s: *const c_char) -> c_double {
    println!("{:?}", *s);
    libc::atof(s)
}

#[repr(C)]
pub struct test_struct {
    integer: c_int,
    f: c_double,
}

#[no_mangle]
pub extern "C" fn handle_struct(tup: test_struct) -> test_struct {
    test_struct {
        integer: tup.integer + 3,
        f: tup.f,
    }
}

#[no_mangle]
pub extern "C" fn count_char(s: *const c_char) -> c_uint {
    let c_str = unsafe {
        assert!(!s.is_null());
        std::ffi::CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    println!("{}", r_str);
    r_str.chars().count() as u32
}
