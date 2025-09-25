// trait merupakan sekumpulan method yang bisa dimiliki oleh struct, enum, primitive.
// dengan trait kita bisa mendefinisikan perilaku yang  harus dimiliki suatu tipe
// trait bisa digunakan sebagai parameter dan return
use crate::Person;

trait Hewan{
    fn bersuara(&self);

    fn tidur(&self) { // kita bisa memberi default implementation
        println!("Zzzzz");
    }
}
// trait bisa mewarisi trait lain jadi SmartHewan harus mengikuti aturan Hewan
trait SmartHewan: Hewan {
    fn smart(&self) {
        println!("Berbicara bahasa manusia");
    }
}

struct Ayam;
struct Kucing;
struct Kakatua;

// hewan harus bisa bersuara untuk ayam
impl Hewan for Ayam {
    fn bersuara(&self) { // bersuara akan menjadi wajib
        println!("kok ko kok kok");
    }
    // fn berjalan(&self) { // berjalan tidak bisa karena belum didefinisikan di trait
    //     println!("Ayam: berjalan");
    // }
}
impl Hewan for Kucing {
    fn bersuara(&self) {
        println!("miaw miaw miaw");
    }
}
impl Hewan for Kakatua{
    fn bersuara(&self) {
        println!("ngopi");
    }
}
impl SmartHewan for Kakatua {} // Kakatua harus mengimplementasikan Hewan terlebih dahulu

#[test]
fn main() {
    let a = Ayam;
    let k = Kucing;
    let kakatua = Kakatua;

    a.bersuara();
    a.tidur(); // default bisa digunakan karena sudah didefinisikan di trait walaupun tidak didefinisikan di Ayam
    k.bersuara();
    kakatua.bersuara();
    kakatua.smart();
}

trait CanSayHello {
    fn say_hello_t(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}
trait CanSayGoodBye {
    fn say_good_bye(&self, name:&str) -> String;
}

impl CanSayHello for Person {
    fn say_hello_t(&self) -> String {
        format!("Hi {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hi {} my name is {}", name, self.first_name)
    }
}
impl CanSayGoodBye for Person {
    fn say_good_bye(&self, name: &str) -> String {
        format!("Goodbye {}", name)
    }
}

// trait sebagai parameter
fn say_hello_trait(value: &impl CanSayHello, name: &str) {
    println!("{}", value.say_hello_t());
    println!("{}", value.say_hello_to(name));
}

// menerapkan lebih dari satu trait
fn hello_and_googbye(value: &(impl CanSayHello + CanSayGoodBye), name: &str) {
    println!("{}", value.say_hello_t());
    println!("{}", value.say_good_bye(name));
}
#[test]
fn test_trait(){
    let person = Person {
        first_name: String::from("Arbath"),
        last_name: String::from("Abdurrahman"),
        age:20,
    };
    say_hello_trait(&person, "Anto");
    println!("{}",CanSayHello::say_hello_to(&person, "Bambang"));
    hello_and_googbye(&person, "Budi");
}