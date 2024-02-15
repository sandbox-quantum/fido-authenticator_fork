#![no_std]

extern crate getrandom;

use getrandom::Error;

pub fn always_fail(_buf: &mut [u8]) -> Result<(), Error> {
    // TODO using trussed.random_bytes
    Ok(())
}
