extern crate nix;

use std::env;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

use nix::fcntl::{splice, SpliceFFlags};
use nix::unistd::pipe;

fn main() {
    for path in env::args().skip(1) {
        let input = File::open(&path).expect(&format!("fcat: {}: No such file or directory", path));
        let (rd, wr) = pipe().unwrap();
        let stdout = io::stdout();
        let _handle = stdout.lock();

        loop {
            let res = splice(
                input.as_raw_fd(),
                None,
                wr,
                None,
                16384,
                SpliceFFlags::empty(),
            ).unwrap();

            if res == 0 {
                break;
            }

            let _res = splice(
                rd,
                None,
                stdout.as_raw_fd(),
                None,
                16384,
                SpliceFFlags::empty(),
            ).unwrap();
        }
    }
}