
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
lazy_static! {
    static ref MEMORY: Mutex<Vec<u32>> = {
        let m = Mutex::new(Vec::new());
        m.lock().unwrap().push(0);
        m
    };
}

fn push(x: u32) {
	MEMORY.lock().unwrap().push(x);
}

fn get(x: usize) -> u32 {
	MEMORY.lock().unwrap()[x]
}

fn main() {
	push(2);
    println!("{}", get(1));
}
