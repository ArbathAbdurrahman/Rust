// kita bisa membuat None atau Undefined menggunakan option

use core::option::Option;

fn double(value:Option<i32>) -> Option<i32>{ // value tidak wajib ada
    match value {
        None => None, // ini jika data kosong
        Some(value) =>Some(value * 2),
    }
}

#[test]
fn test_option(){
    let result = double(Some(42));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}