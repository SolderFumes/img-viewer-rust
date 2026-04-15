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

    //header now has the header.
    // We need to extract the size of the entire file and the pointer to the actual image.
    // .expect() unpacks the Result and panicks if there's an error.
    let size_slice: [u8; 4] = header[2..=5].try_into().expect("Slice shoudl be 4 elements long.");

    print!("Size header reads: ");
    for b in &size_slice {
        print!("{:02X} ", b);
    }
    println!("");

    let size: u32 = bytes_to_decimal(&size_slice);

    println!("File is {} bytes long.", size);
    // size is now the correct decimal size. Let's get the pointer
    let img_pointer_arr: [u8; 4] = header[10..=13].try_into().expect("Slice should be exactly 4 elements long.");
    let img_pointer: u32 = bytes_to_decimal(&img_pointer_arr);
    

    let img_vec: Vec<u8> = Vec::from(header);

    return Ok(img_vec);


}

fn bytes_to_decimal(slice: &[u8; 4]) -> u32 {
    let mut dec: u32 = 0;
    for byte in (0..=3).rev() {
        dec += slice[byte] as u32 * (u32::pow(256_u32, byte as u32));
        //println!("The byte is {0}. Assigning value {1}. Size is currently {2}", size_slice[byte], size_slice[byte] as u32 * (u32::pow(256_u32, byte as u32)), size);
    }
    dec
}
