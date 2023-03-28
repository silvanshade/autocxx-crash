mod gen;

use autocxx::prelude::*;
use cxx::UniquePtr;

fn main() {
    println!("start");
    run(); // crashes here
    println!("done");
}

fn run() {
    let str0 = "string";
    let str1 = "another string";
    let ptr0 = UniquePtr::emplace(gen::Test::new(str0));
    let ptr1 = UniquePtr::emplace(gen::Test::new(str1));
    println!("0: {}", ptr0.get_string());
    println!("1: {}", ptr1.get_string());
    moveit!(let mut ref0 = &move *ptr0);
    moveit!(let mut ref1 = &move *ptr1);
    println!("0: {}", ref0.get_string());
    println!("1: {}", ref1.get_string());
    println!("swap");
    core::mem::swap(&mut *ref0, &mut *ref1);
    println!("0: {}", ref0.get_string());
    println!("1: {}", ref1.get_string());
}
