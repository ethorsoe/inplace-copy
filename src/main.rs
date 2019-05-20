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

fn copy_block(fhandles: &mut Vec<BufferedFile>, offset: u64) -> bool {
	for bf in &mut*fhandles { unsafe {
		libc::pread64(bf.file.as_raw_fd(),
			(bf.buffer[0..BUFFER_SIZE].as_mut_ptr()) as *mut libc::c_void,
			BUFFER_SIZE as libc::size_t, offset as libc::off64_t);
	}}
	
	return fhandles[0].buffer[0..BUFFER_SIZE] != fhandles[1].buffer[0..BUFFER_SIZE];
}

fn main() {
	if env::args_os().count() != 3 {
		panic!("usage: {} <infile> <outfile>", env::args_os().nth(0).unwrap().into_string()
			.unwrap());
	}
	let mut args_iter = env::args_os().into_iter();
	args_iter.next();
	let mut fhandles : Vec<BufferedFile> = args_iter.map(|s| BufferedFile {
		file: File::open(&s).expect((String::from("No such file ") + s.into_string()
			.expect("File with unprintable name can't be opened").as_str()).as_str()),
		buffer: [0; BUFFER_SIZE],
	}).collect();
	let first_fd = fhandles[0].file.as_raw_fd();
	println!("firstfd {}", &first_fd);
	let mut counter: u64 = 0;
	while copy_block(&mut fhandles, counter) {
		counter += BUFFER_SIZE as u64;
	}
}
