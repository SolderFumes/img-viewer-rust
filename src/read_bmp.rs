use std::fs::File;
use std::io::prelude::*;

pub fn read_bmp(target_file: &str) -> Result<Vec<u8>, &'static str> { 
    let file_res = File::open(target_file); //File::open returns a Result<()> so we use ?
                                             //to unpack the actual return value. 
    let mut file: File;
    if file_res.is_err() {
        return Err("File opening failed.");
    } else {
        file = file_res.unwrap()
    }
    let mut header: [u8; 52] = [0; 52];
    let res = file.read_exact(&mut header); // This will read until buf is full.
    if res.is_err() {
        return Err("File couldn't be read");
    }

    dbg!(header);

    let img_vec: Vec<u8> = Vec::from(header);

    return Ok(img_vec);


}
