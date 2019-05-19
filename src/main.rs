#![ macro_use ]

pub extern crate nix;
pub extern crate libc;

use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

const BUFFER_SIZE: usize = 4096;

struct BufferedFile {
	file: File,
	buffer: [u8; BUFFER_SIZE],
}

fn main() {
	if env::args_os().count() != 3 {
		panic!("usage: {} <infile> <outfile>", env::args_os().nth(0).unwrap().into_string().unwrap());
	}
	let mut args_iter = env::args_os().into_iter();
	args_iter.next();
	let fhandles : Vec<BufferedFile> = args_iter.map(|s| BufferedFile {
		file: File::open(&s).expect((String::from("No such file ") + s.into_string().expect("File with unprintable name can't be opened").as_str()).as_str()),
		buffer: [0; BUFFER_SIZE],
	}).collect();
	let first_fd = fhandles[0].file.as_raw_fd();
	println!("firstfd {}", &first_fd);
}
