
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::char;

lazy_static! {
    static ref MEMORY: Mutex<Vec<i8>> = {
        let m = Mutex::new(Vec::new());
        m.lock().unwrap().push(0);
        m
    };
}

fn push() {
	MEMORY.lock().unwrap().push(0);
}

fn get(x: &usize) -> i8 {
	MEMORY.lock().unwrap()[*x]
}

fn update(x: &usize, val: i8) {
//	println!("usize = {} val = {} ", x, val);
	MEMORY.lock().unwrap()[*x] += val;
}

fn main() {
	let now = Instant::now();
	let mut f = File::open("src/bf.bf").expect("file not found");
	let mut contents = String::new();

	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	println!("With text: {}", contents);

	let mut ptr: usize = 0;

	run(&mut ptr, &contents);

	for i in MEMORY.lock().iter() {
		println!("{:?}", i);
	}

	println!("{} microsec", now.elapsed().as_micros());
}


fn run(ptr: &mut usize, contents: &String) {
//	println!("{}", contents);
	let mut jump: i32 = -1;

	for (i, c) in contents.chars().enumerate() {
		if jump != -1 && i < jump as usize {
			continue;
		}
		if c == '+' {
			update(ptr, 1)
		}
		else if c == '-' {
			update(ptr, -1)
		}
		else if c == '>' {
			*ptr += 1;
			if MEMORY.lock().unwrap().len() - 1 < *ptr {
				push();
			}
		}
		else if c == '<' {
			*ptr -= 1;
		}
		else if c == '.' {
			print!("{}", get(ptr) as u8 as char)
		}
		else if c == '!' {
			print!("{}", get(ptr))
		}
		else if c == '[' {
			let mut skiploop = 1;
			let mut endindex = i.clone() + 1;
			while get(&ptr) > 0 {
//				println!("pointeur {} -> {}", ptr, get(ptr));
				while skiploop != 0 {
					if contents.chars().nth(endindex).unwrap() == '[' {
						skiploop += 1;
					}
					else if contents.chars().nth(endindex).unwrap() == ']' {
						skiploop -= 1;
					}
					endindex += 1;
				}
				run(ptr, &contents.chars().skip(i + 1).take(endindex - i - 2).collect());
			}
			jump = endindex as i32;
		}
	}
}
