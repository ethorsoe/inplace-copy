extern crate blkid;
extern crate btrfs;

use blkid::dev::Devs;
use blkid::cache::Cache;
use btrfs::linux;
use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn do_blkid(uuid: &str) {
	let cache = Cache::new().unwrap();
	for device in Devs::new(&cache) {
		let mut is_btrfs = false;
		let mut has_right_uuid = false;
		
		for (tag, value) in device.tags() {
			if tag == "TYPE" &&  value == "btrfs" {
				is_btrfs = true;
			} else if tag == "UUID" && value == uuid {
				has_right_uuid = true;
			}
		}
		if is_btrfs  && has_right_uuid {
			let path = device.name();
			println!("identified: {}", path.to_string_lossy());
		} 
	}
}

fn main() {
	if env::args_os().count() != 2 {
		panic!("usage: {} <path>", env::args_os().nth(0).unwrap().into_string().unwrap());
	}
	let path = env::args_os().nth(1).unwrap().into_string().unwrap();
	let fhandle = File::open(&path).unwrap();
	let root_fd = fhandle.as_raw_fd();
	let fs_info = linux::get_filesystem_info(root_fd).unwrap();
	let uuid = fs_info.filesystem_id.hyphenated().to_string();

	do_blkid(&uuid);
}
