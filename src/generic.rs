/* GENERIC
    - kita bisa membuat function, struct, enum, method, dan trait yang tipe datanya bisa diubah
    -
 */
use crate::Person;
use crate::trait_concept::{CanSayGoodBye};

// generic di stuct
struct Point<T = i32> { // default dengan =
    x: T,
    y: T,
}

#[test]
fn test_generic_struct(){
    let integer: Point<i32> = Point::<i32>{x: 1, y: 2}; // eksplisit
    let float = Point {x: 1.0, y: 2.0}; // implisit
    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y);
}

// generic di enum
enum Value<T> {
    NONE,
    VALUE(T),
}
#[test]
fn test_generic_enum(){
    let value = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("NONE");
        }
        Value::VALUE(value) => {
            println!("x = {}", value);
        }
    }
}

// Generic type bound dengan trait
// kita bisa membatasi type apa yang dibolehkan contohnya ini hanya tipe Person dari CanSayGoodbye saja yang diperbolehkan

struct Hi<T> where T: CanSayGoodBye{ // partial order where
    value: T,
}
#[test]
fn test_hi(){
    let hi = Hi::<Person>{
        value: Person {
            first_name: String::from("Arbath"),
            last_name: String::from("Abdurrahman"),
            age: 20
        }
    };
    println!("{}", hi.value.first_name);
}

fn min<T: PartialOrd>(a: T, b: T) -> T{
    if a < b {
        a
    } else {
        b
    }
}
#[test]
fn test_min(){
    let result = min::<i32>(10,20);
    println!("{}", result);
}

// Generic di method
impl <T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}
#[test]
fn test_generic_methods(){
    let point = Point {x:10, y:20};
    print!("{}", point.get_x());
    print!("{}", point.get_y());
}

// generic di trait
trait GetValue<T> {
    fn get_value(&self) -> &T;
}
impl<T> GetValue<T> for Point<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}