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

#[test]
fn if_expression(){
    // percabangan di rust sama dengan bahasa pemograman lain.
    // tetapi bisa digabung dengan variabel let tanpa pakai return ;D
    let value = 10;
    let result: &str = if value > 10 {
        "value > 10"
    } else if value == 10 {
        "value = 10"
    } else {
        "value < 10"
    };
    println!("{}", result);
}

#[test]
fn looping(){
    // looping sama juga dengan pemograman lain
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("counter: {}", counter);
    }

    // menggunakan let
    let mut batas = 0;
    let result = loop {
        batas = batas + 1;
        if batas > 10 {
            break batas * 2;
        }
    };
    println!("result: {}", result);

    let mut i = 0;
    loop {
        if i >= 5 { break; }
        println!("i = {}", i);
        i += 1;
    }
}

#[test]
fn loop_label(){
    // nested loop menggunakan label ditandai dengan '
    let mut number = 1;
    'terluar: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'terluar; // akan menghentikan loop lapisan terluar
            }
            println!("{} x {} = {}",number, i, number * i);

            i += 1;
            if i > 10 {
                break; // akan menghentikan loop lapisan terdalam
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop(){
    // while untuk print bilangan genap sama seperti loop
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn for_loop(){
    // versi array
    let arr: [i32;10] = [1,20,30,40,50,60,70,80,90,100];
    let mut index = 0;


    // otomatis
    for i in arr {
        println!("{}", i);
    }

    // kalau manual
    while index < arr.len(){
        println!("{}", arr[index]);
        index += 1;
    }
}

#[test]
fn range(){
    let arr: [&str;5] = ["A", "B", "C", "D", "E"];
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("{}", arr[i]);
    }

    // versi in range
    for k in 0..10 {
        println!("k: {}", k);
    }
}

#[test]
fn range_inclusive() {
    // kalau ingin bagian akhir diambil
    let arr: [&str; 5] = ["A", "B", "C", "D", "E"];
    let range = 0..=4;

    for i in range {
        println!("{}", arr[i]);
    }
}

fn say_hello(first_name: &str, last_name: &str){
    println!("Hello {} {}.", first_name, last_name);
}

fn factorial_loop(n: i32) -> i32{
    // fungsi dengan return
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn main_func(){
    say_hello("Arbath", "Abdurrahman");
    say_hello("Joko", "Sulistio");
    let result:i32 = factorial_loop(9);
    println!("result: {}", result);
}

fn recursive(value: String, times:u32){
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }
    recursive(value, times-1);
}

#[test]
fn recursive_func(){
    recursive(String::from("Arbath Abdurrahman"), 3);
}

/* OWNERSHIP IN FUNCTION
- parameter berbasis stack ownernya akan di copy
- parameter berbasis heep ownernya akan dipindah ke function
    jadi saat function berhasil dijalankan maka variabel yang dimasukkan ke parameter akan tidak bisa diakses.
*/
fn print_number(number:i32){
    println!("{}", number);
}
fn hi(name:String){
    println!("{}", name);
}
#[test]
fn func_owner(){
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Arbath Abdurrahman");
    hi(name);
    // println!("name: {}", name); // ini akan error karena name jadi milik hi()
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name) // returnkan kembali ownership
}

#[test]
fn func_full_name(){
    let first_name = String::from("Arbath");
    let last_name = String::from("Abdurrahman");

    let name = full_name(first_name, last_name); // kirim ownership ke full_name()
    println!("Namaku {}", name); // karena ownership telah di return full_name() maka sekarang full name jadi milik name

    // ini tidak bisa karena sudah ditransfer ke full_name()
    // println!("{}", first_name);
    // println!("{}", last_name);
}

fn not_claim(first_name: String, last_name: String) -> (String, String, String) {
    let full_name: String = format!("{} {}", first_name, last_name);

        (first_name, last_name, full_name) // kembalikan ownership dalam format tuple
}

#[test]
fn func_full_name_return(){
    let first_name = String::from("Arbath");
    let last_name = String::from("Abdurrahman");

    let (first_name, last_name, name) = not_claim(first_name, last_name); // kirim ownership ke not_claim()
    println!("Namaku {}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

/*
REFERENCE (&)
- bebas  dibuat sebanyak apapun ini seperti memberikan akses ke heap tanpa pindah owner
- lifetime reference mengikuti lifetime yang di pinjami akses
    jadi saat variabel yang dipinjami akses data heap dihapus data masih tetap ada selama owner ada
 */
fn ref_full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name) // sudah reference
}

#[test]
fn ref_func_full_name(){
    let first_name = String::from("Arbath");
    let last_name = String::from("Abdurrahman");

    let name = ref_full_name(&first_name, &last_name); // kirim reference (&) ke ref_full_name()
    println!("Namaku {}", name);

    // ini bisa karena ref_full_name() meminjam data bukan pidah kepemilikan
    println!("{}", first_name);
    println!("{}", last_name);
}

/* BORROWING
  - karena reference itu meminjam maka peminjam secara default tidak bisa mengubah nilai yang dipinjam
  - dalam satu waktu hanya boleh ada satu mutable reference agar bisa mengubah value (&mut)
*/
fn change_val(val: &mut String) {
    val.push_str("Ubah\n");
}
#[test]
fn func_change_val() {
    // pastikan owner mutable agar yang dipinjami dapat mengubah data
    let mut val = String::from("Arbath");
    // change_val(&mut val); // kirim sebagai mutable juga
    let val_borrow = &mut val; // ini boleh karena satu dan masih satu waktu
    // let val_borrow2 = &mut val; // ini tidak boleh karena sudah beda waktu
    // let val_borrow2 = &val; // ini juga tidak boleh
    change_val(val_borrow);
    // change_val(val_borrow2);

    println!("val: {}", val);
}

/* DANGLING POINTER
    - reference menunjuk data yang tidak ada di memory (-> &)

fn dang_full_name(first_name: &String, last_name: &String) -> &String {
    let name format!("{} {}", first_name, last_name) // sudah reference
    return &name;
}
 */

#[test]
fn slice(){ // reference ke sebagian data menggunakan range (..)
    // semua slice bertipe fix makanya ownernya di copy
    let arr: [i32;10] = [1,2,3,4,5,6,7,8,9,10];
    let slice1: &[i32] = &arr[..];
    println!("slice1: {:?}", slice1);

    let slice2 = &arr[3..];
    println!("slice2: {:?}", slice2);

    let slice3 = &arr[..2];
    println!("slice3: {:?}", slice3);

    let slice4 = &arr[4..8];
    println!("slice4: {:?}", slice4);
}

#[test]
fn str_slice(){ // ownership masih pada name
    let name: String = String::from("Arbath Abdurrahman");
    let first_name: &str = &name[..6];
    println!("first_name: {}", first_name);

    let last_name: &str = &name[7..];
    println!("last_name: {}", last_name);
}

/* STRUCT
    - hampir seperti class tapi tanpa inheritance
 */
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}
/* METHOD
    - gunakan self dan reference agar tidak di claim oleh method
 */
impl Person {
    fn say_hello(&self, name:&str){
        println!("Hello {} my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person){
    println!("first_name: {}\nlast_name: {}\nage: {}", person.first_name, person.last_name, person.age);
}
#[test]
fn struct_person(){
    let last_name = String::from("Abdurrahman");

    let person: Person = Person {
        first_name: String::from("Arbath"),
        age: 20,
        last_name, // ownership akan berpindah ke Person
    };
    print_person(&person);
    person.say_hello("Budi");

    // Struct Update Syntax
    // hati-hati kalau ada value heap harus .clone()
    let person2 = Person{..person}; // ownership person pindah ke person2 kalau data heap
    print_person(&person2);
}

struct  GeoPoint(f64, f64);
struct Nothing; // struct tanpa field
impl GeoPoint {
    // assosiated function
    // ini function tapi bukan sebagai method aksesnya menggunakan (struct::fn)
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}
#[test]
fn tuple_struct(){
    let geo_point = GeoPoint(123.3, 456.2);
    println!("{}",geo_point.0); // akses seperti tiple
    println!("{}",geo_point.1);

    // gunakan _ jika tidak digunakan
    let _nothing = Nothing;
    let _nothing2 = Nothing{};
}

#[test]
fn assosiated_function(){
    let geo_point = GeoPoint::new(123.3, 456.2);
    println!("{}",geo_point.0);
}

/* ENUM
    - tipe data yg dibuat untuk mengumpulkan beberapa kemungkinan
    - bisa  ditambah method seperti struct
 */
enum Level {
    Regular,
    Premium,
    Platinum,
}
#[test]
fn test_enum(){
    let _level1 = Level::Platinum;
    let _level2 = Level::Premium;
    let _level3 = Level::Regular;
}

enum  Payment {
    CreditCard(String),
    BankTransfer(String, String),
    Ewallet(String, String),
}
impl Payment {
    fn pay(&self, amount:i32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::Ewallet(wallet, number) => {
                println!("Paying with wallet {} {} amount {}", wallet,number, amount);
            }
        }
    }
}
#[test]
fn test_payment(){
    let _payment: Payment = Payment::CreditCard(String::from("3423234"));
    _payment.pay(10);

    let _payment2: Payment = Payment::BankTransfer(String::from("BankTransfer"), String::from("3423234"));
    _payment2.pay(50);

    let _payment3: Payment = Payment::Ewallet(String::from("Ewallet"), String::from("3423234"));
    _payment3.pay(100);
}

/* PATTERN MATCHING
    - match seperti percabangan match case py pengganti if else di rust bisa digunakan di enum
    - bisa juga megambil data di enum
    - ignore bisa menggunakan (..) atau (_)
 */

#[test]
fn match_enum(){
    let level = Level::Premium;
    match level {
        Level::Platinum => {
            println!("Platinum");
        },
        Level::Premium => {
            println!("Premium");
        }
        Level::Regular => {
            println!("Regular");
        }
    }
}

#[test]
fn match_value(){
    let name = "Paijo";

    match name {
        "Budi" => {
            println!("Hello Budi");
        }
        "Bambang" | "Paijo" => { // bisa tambahkan pipe
            println!("Hello BOS");
        }
        // 80..=100 => { // bisa juga pakai range
        //     println!("Hello 80 to 100");
        // }
        other => {
            println!("Hello {}", other);
        }
    }
}

// type alias
type Age = u32;
type KTP = String;
struct Customor {
    id: KTP,
    name: String,
    age: Age,
}
#[test]
fn type_alias(){
    let customor = Customor{
        id: String::from("4242435"),
        name: String::from("Arbath"),
        age: 20,
    };
    println!("customor id: {}", customor.id);
    println!("customor name: {}", customor.name);
    println!("customor age: {}", customor.age);
}