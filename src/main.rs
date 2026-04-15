// RUST IMAGE VIEWER
// RESTRICTIONS: ONLY STD LIBRARY
// 
// IMPLEMENTATION STEPS:
// 1) READ FILE
// 2) OPEN OS WINDOW
// 3) PAINT WINDOW 
// 4) ???
// 5) PROFIT
//
mod read_bmp;

fn main() {
    let res = read_bmp::read_bmp("/home/solderfumes/personal/img-viewer-rust/1bpp-320x240.bmp");
    if res.is_err() {
        println!("What happpened??");
        let _ = dbg!(res);
    }
}
