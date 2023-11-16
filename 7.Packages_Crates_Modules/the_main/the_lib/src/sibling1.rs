use crate::sibling2;

pub mod s1 {
    pub fn hello_s1() {
        println!("hello s1!");
    }
}

pub fn hello() {
    sibling2::s2::hello_s2();
}
