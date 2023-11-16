use crate::sibling1;

pub mod s2 {
    pub fn hello_s2() {
        println!("hello s2!!");
    }
}

pub fn hello() {
    sibling1::s1::hello_s1();
}
