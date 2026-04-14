// Read a Bitmap image as a Byte array. 
// How do we do this?
// There is a function file_var.read(&mut buffer[..]) which returns a Result<usize> (usize is a mem address/ptr). The result (when unwrapped) tells you how many bytes were used.
// So we can read bytes up until we've filled the buffer, then store that data in a Vec<u8> (more
// permanent collection) then read from (the start of the file + n) until we fill up the buffer
// and repeat :3.
//
//
// Turns out there's a better way. Since there's a file size in bytes in the BMP header, we don't
// even have to use a buffer except for the header. We read the header (54 bytes) into a buffer,
// find the total length, then make a Vec<u8> with that many bytes minus 54. Now, we can read
// directly into that Vec<u8> without having to use any buffer since we know the exact length.
//
// We do need to keep the entire file in memory.
//
use std::fs::File;
use std::io::prelude::*;

fn read_bmp(target_file: &str) -> () { 
    let mut file = File::open(target_file)?; //File::open returns a Result<()> so we use ?
                                             //to unpack the actual return value. 
    let mut buf = [52]

}
