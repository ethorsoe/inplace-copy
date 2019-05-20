#![ macro_use ]

pub extern crate nix;
pub extern crate libc;

use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

const BUFFER_SIZE: usize = 4096;

struct BufferedFile {
	file: File,
	last_read: u64,
	buffer: [u8; BUFFER_SIZE],
}

fn copy_block(fhandles: &mut Vec<BufferedFile>, offset: u64) -> bool {
	for bf in &mut*fhandles { unsafe {
		bf.last_read = libc::pread64(bf.file.as_raw_fd(),
			(bf.buffer[0..BUFFER_SIZE].as_mut_ptr()) as *mut libc::c_void,
			BUFFER_SIZE as libc::size_t, offset as libc::off64_t) as u64;
	}}
	let last_read = fhandles[0].last_read;
	let olast_read = fhandles[1].last_read;	
	if BUFFER_SIZE as u64 != last_read {
		eprintln!("Pread return {} not {} at {}", last_read, BUFFER_SIZE, offset);
	}
	if last_read < olast_read {
		eprintln!("Pread return {} of outfile larger than infile return {}",
			olast_read, last_read);
	}
	if fhandles[0].buffer[0..last_read as usize] != fhandles[1].buffer[0..last_read as usize] ||
			olast_read < last_read {
		unsafe {
			let written = libc::pwrite64(fhandles[1].file.as_raw_fd(),
				(fhandles[0].buffer[..].as_mut_ptr()) as *mut libc::c_void,
				last_read as libc::size_t, offset as libc::off64_t) as u64;
			if last_read != written {
				panic!("Write of {} bytes to outfile at offset {} returned {}",
					last_read, offset, written);
			}
		}
	}
	return last_read == BUFFER_SIZE as u64;
}

fn main() {
	if env::args_os().count() != 3 {
		panic!("usage: {} <infile> <outfile>", env::args_os().nth(0).unwrap().into_string()
			.unwrap());
	}
	let mut args_iter = env::args_os().into_iter();
	args_iter.next();
	let mut fhandles : Vec<BufferedFile> = args_iter.map(|s| BufferedFile {
		file: std::fs::OpenOptions::new().read(true).write(true).open(&s)
			.expect((String::from("No such file ") + s.into_string()
			.expect("File with unprintable name can't be opened").as_str()).as_str()),
		last_read: 0,
		buffer: [0; BUFFER_SIZE],
	}).collect();
	let mut counter: u64 = 0;
	while copy_block(&mut fhandles, counter) {
		counter += BUFFER_SIZE as u64;
	}
}
