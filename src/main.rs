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

/*
Tipe data rust:
Scalar = nilai tunggal
Compound = bisa lebih dari satu
 */

// TIPE DATA SCALAR
#[test]
fn integer(){
    // rust sudah otomatis mendeteksi tipe data seperti python tetapi bisa juga secara eksplisit sebutkan tipe data
    let implisit = 12;
    let eksplisit: i16 = -12; // ini integer berukuran 16 bit
    let unsignd: u32 = 24; // ini hanya menerima bilangan  positif

    println!("implisit: {}", implisit);
    println!("eksplisit: {}", eksplisit);
    println!("unsignd: {}", unsignd);
}

#[test]
fn floating(){
    // float diawali dengan fbit dan hanya 32 dan 64 bit
    let implisit = 12.34;
    let eksplisit: f32 = -123.45;

    println!("implisit: {}", implisit);
    println!("eksplisit: {}", eksplisit);
}

#[test]
fn konversi(){
    // kita bisa melakukan konversi tipe data asalkan masih muat dengan 'as'
    let a: i8 = 10;
    println!("a: {}", a);

    let b: i16 = a as i16;
    println!("b: {}", b);

    let c: i32 = b as i32;
    println!("c: {}", c);

    // kalau tidak muat maka akan terjadi integer overflow
    let d: i64 = 1000000000;
    println!("d: {}", d);

    let e: i16 = d as i16;
    println!("e: {}", e); // akan tidak sesuai keinginan
}