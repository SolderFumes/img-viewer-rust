use std::fs::File;
use std::io::prelude::*;
pub fn read_bmp(target_file: &str) -> Result<Vec<u8>, std::io::Error> { 
    let mut file: File = File::open(target_file)?; //File::open returns a Result<()> so we use ?
                                             //to unpack the actual return value. 
    let mut header: [u8; 52] = [0; 52];
    file.read_exact(&mut header)?; // This will read until buf is full.

    //header now has the header.
    // We need to extract the size of the entire file and the pointer to the actual image.
    // .expect() unpacks the Result and panicks if there's an error.
    let size_slice: [u8; 4] = header[2..=5].try_into().expect("Slice shoudl be 4 elements long.");

    print!("Size header reads: ");
    for b in &size_slice {
        print!("{:02X} ", b);
    }
    println!("");

    //52 bytes are header so subtract that from size :) 
    let size: u32 = bytes_to_decimal(&size_slice) - 52;

    println!("File is {} bytes long.", size);
    // Thanks to file cursors, we don't need to find the image pointer as we have already "consumed"
    // the header with file.read_exact() and the entire rest of the file should be image data!
    //let img_pointer_arr: [u8; 4] = header[10..=13].try_into().expect("Slice should be exactly 4 elements long.");
    //let img_pointer: u32 = bytes_to_decimal(&img_pointer_arr);
    
    // When creating this blank pre-sized vector, we need to convert size (u32) to usize (either u32
    // or u64 depending on system architecture.) This is safe to do as usize will never be smaller
    // than a u32, preventing overflow.
    let mut img_vec: Vec<u8> = vec![0; size as usize];
    file.read_exact(&mut img_vec).expect("Did not read image data correctly.");

    return Ok(img_vec);
}

// expects a 4-byte slice and converts it to decimal.
// I'm not exactly sure how I can dynamically change the return and arg types so... for now I only
// need 4 bytes' worth of conversion.
fn bytes_to_decimal(slice: &[u8; 4]) -> u32 {
    let mut dec: u32 = 0;
    for byte in (0..=3).rev() {
        dec += slice[byte] as u32 * (u32::pow(256_u32, byte as u32));
        //println!("The byte is {0}. Assigning value {1}. Size is currently {2}", size_slice[byte], size_slice[byte] as u32 * (u32::pow(256_u32, byte as u32)), size);
    }
    dec
}
