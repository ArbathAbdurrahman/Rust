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

#[test]
fn numeric_operators(){
    let a = 25;
    let b = 2;

    println!("a: {}, b: {}", a, b);
    println!("a + b: {}", a + b);
    println!("a - b: {}", a - b);
    println!("a * b: {}", a * b);
    println!("a / b: {}", a / b);
    println!("a % b: {}", a % b);
}

#[test]
fn augmented_asignment(){
    // operasi ke variabel yang sama
    let mut a = 25; // ganti dulu variabel menjadi mutable
    let b = 2;
    println!("a: {}, b: {}", a, b);

    a += b;
    println!("a = a += b :{}", a);

    a -= b;
    println!("a = a -= b :{}", a);

    a *= b;
    println!("a = a *= b :{}", a);

    a /=b ;
    println!("a = a /= b :{}", a);

    a %= b;
    println!("a = a %= b :{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("a: {}", a);
    println!("b: {}", b);
}

#[test]
fn comparison(){
    let a = 25;
    let b = 2;
    println!("a: {}, b: {}", a, b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);
    println!("a == b: {}", a == b);
    println!("a != b: {}", a !=b);
}

#[test]
fn boolean_operators(){
    let a = true;
    let b = false;
    println!("a: {}, b: {}", a, b);
    println!("a && b: {}", a && b);
    println!("a || b: {}", a || b);
    println!("!a: {}", !a);
}

#[test]
fn char(){
    // kalau char pakai petik satu ('') hanya untuk satu karakter
    let a: char = 'a';

    println!("a: {}", a);
}

// TIPE DATA COMPOUND
#[test]
fn tuple(){
    // tuple merupakah kumpulan lebih dari satu tipe data dengan jumlah fix
    // tentukan dahulu tipe data yang dapat ditampung
    let mut data: (i32, f64, bool) = (10, 3.14, true);
    let pi = data.1;
    data.2 = false; // mengubah data dalam tuple

    // print pakai :?
    println!("data: {:?}", data);
    println!("ini sepuluh => {:?}", data.0);
    println!("ini pi => {}", pi);

    // descructuring tuple
    let (x, y, z) = data;
    println!("x: {}, y: {}, z: {}", x, y, z);
    // tambahkan _ jika tidak mau akses
    let (a,_,c) = data;
    println!("a: {}, c: {}", a,c);

    // memanggil tuple kosong
    unit();
    let kosong: () = unit();
    println!("kosong => {:?}", kosong);

    let test_unit: () = ();
    println!("test_unit => {:?}", test_unit);
}

#[test]
fn unit(){
    println!("ini tuple kosong.");
}

#[test]
fn array(){
    // array hanya meyimpan satu tipe data yang sama dan berukuran fix
    let mut array: [i32;5]  = [1,2,3,4,5];
    array[1] = 6;
    println!("array: {:?}", array);
    println!("index ke-{:?}", array[0]);
    println!("index ke-{:?}", array[2]);
    println!("index ke-2 = {:?}", array[1]);
    println!("panjang array = {:?}", array.len());
}

#[test]
fn array_2d(){
    // array 2 dimensi alias array didalam array
    let matrix: [[i32;3];2] = [
        [1,3,3],
        [3,4,6]
    ];

    println!("matrix: {:?}", matrix);
    println!("array pertama: {:?}", matrix[0]);
    println!("array pertama index pertama: {:?}", matrix[0][0]);
    println!("array kedua index kedua: {:?}", matrix[1][1]);
}

const MINIMUM: i32 = 0;
#[test]
fn constant(){
    // constant harus explisit dan pakai huruf besar snake upercase dan bisa dibuat diluar scope fn
    const MAXIMUM: i32 = 100;
    println!("min: {}", MINIMUM);
    println!("max: {}", MAXIMUM);
}

#[test]
fn variable_scope(){
    let nama = "Arbath";
    println!("nama: {}", nama);

    {
        println!("inner nama: {}", nama);
        let age = 20;
        println!("inner age: {}", age);
    }
    // println!("outer age: {}", age); ini diluar scope
}

/*
MANAJEMEN MEMORI
- ketika data sudah fix maka rust akan menyimpan stack (LIFO)
- ketika data tidak fix maka rust akan menyimpan di dalam Heap
+ heap = kita akan request ke alocator untuk meminta memori dan akan diberikan pointer
+ rust akan menghapus data di stack paling atas dahulu last in first out
    jadi kalau ada variable keluar dari scope maka dia drop untuk akan membebaskan heap tanpa garbage colletor dan manual manajemen memori
*/

#[test]
fn stack_heap(){
    func_a(); // ini akan masuk ke paling bawah
    func_b(); // ini akan masuk ke atas func_a
}

#[test]
fn func_a(){
    let a = 10; // ini akan disimpan kedalam stack
    let b = String::from("Arbath"); // string sisimpan didalam heap dan diakses menggunakan pointer
    println!("a: {}, b: {}", a, b);
}

#[test]
fn func_b(){
    let a = 100;
    let b = String::from("Abdurrahman");
    println!("a: {}, b: {}", a, b);
}

#[test]
fn string(){
    // string slice akan disimpan di stack
    // ini fix dan tidak akan berubah tetapi akan bisa diganti sehingga data masih ada di memori
    let name: &str = " Arbath Abdurrahman ";
    let trim: &str = name.trim();
    println!("name: {}", name);
    println!("trim: {}", trim);

    let mut username: &str = "Arbath";
    println!("username: {}", username);
    username = "Arbath Abdurrahman";
    println!("username: {}", username);
}

#[test]
fn string_type(){
    // string type akan disimpan di heap
    // kita bisa memanipulalsi string dengan method yang telah disediakan
    // selengkapnya baca di https://doc.rust-lang.org/std/primitive.str.html
    let mut name: String = String::from("Arbath ");
    name.push_str("Abdurrahman ");
    println!("name: {}", name);

    let budi = name.replace("Arbath ", "Budi ");
    println!("budi: {}", budi);
}

/* OWNERSHIPS
- setiap value haru punya owner
- hanya boleh ada satu owner dalam satu waktu
- ketika owner keluar scope maka value akan dihapus
 */
#[test]
fn ownership(){
    // a tidak bisa diakses disini
    let a = 10;
    {
        let b = 20;
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi dan terhapus
    println!("{}", a);
}// scope a selesai, a dihapus, a tidak bisa diakses lagi dan otomatis dihapus dari memori

#[test]
fn data_copy(){
    // dia akan mencopy data dan hanya di stack tidak dapat terjadi di data heap
    let a = 10;
    let b = a;
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement(){
    // permindahan kepemilikan dari owner lama ke owner baru dan owner lama akan tidak valid hanya dapat terjadi di heap
    let name = String::from("Arbath");
    // transfer name ke username
    let username = name;
    // println!("name: {}", name); <= ini sudah tidak valid
    println!("name: {}", username);
}

#[test]
fn clone(){
    // copy tapi untuk heap
    let name = String::from("Arbath");
    let username = name.clone();

    println!("{}\n{}", name, username);
}