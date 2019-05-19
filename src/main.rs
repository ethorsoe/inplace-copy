#![ macro_use ]

pub extern crate nix;

use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main() {
	if env::args_os().count() != 3 {
		panic!("usage: {} <infile> <outfile>", env::args_os().nth(0).unwrap().into_string().unwrap());
	}
	let mut args_iter = env::args_os().into_iter();
	args_iter.next();
	let fhandles : Vec<File> = args_iter.map(|s| File::open(&s).expect((String::from("No such file ") + s.into_string().expect("File with unprintable name can't be opened").as_str()).as_str())).collect();
	let first_fd = fhandles[0].as_raw_fd();
	println!("firstfd {}", &first_fd);
}
