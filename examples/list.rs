extern crate bsd_errnos;

use bsd_errnos::*;

pub fn main() {
    for errno in Errno::iter() {
        println!(
            "{}: {}",
            errno.name().unwrap(),
            errno.description().unwrap()
        );
    }
}
