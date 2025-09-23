/*
    run rust dengan cargo test nama_test -- --nocapture

    compiler rust memaksa kita untuk menulis kode secara efisien dan warning ketika ada yang tidak best practice
 */

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello() {
    // variabel di rust imuntable by default dikelurkan dengan cara menambahkan {}

    let nama = "Arbath";
    println!("Hello, {}!", nama);
}

#[test]
fn mutable(){
    // tambahkan mut agar bisa diubah
    let mut nama = "Arbath";
    println!("Hello, {}!", nama);

    nama = "Abdurrahman";
    println!("Hello, {}!", nama);
    // rust bersifat static typing jadi sting tidak bisa diganti integer
    // nama = 100; ini akan error
}

#[test]
fn shadowing(){
    // di rust bisa menggunakan nama variabel yang sama

    let nama = "Arbath";
    println!("Hello, {}!", nama);

    // nama akan tertutup bukan tertimpa karena dia berada di memori yang berbeda
    let nama = "Arbath Abdurahman";
    println!("Hello, {}!", nama);
}

