use crate::third::say_hello as say_hello3;
// gunakan use crate diluar main dan tambah mod third di main
// bisa juga menggunakan super:: untuk naik modul
pub fn say_hello(){
    println!("Hello from first module");
    say_hello3();
}