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

//Opening a window with just the stdlib seems impractical. I would have to create a program that
//asynchronyously interacts with the wayland server. I will instead use a basic crate called
//"show_image" that does exactly that with as few as possible other features. 
use show_image::{ImageView, ImageInfo, create_window, event};

#[show_image::main]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = read_bmp::read_bmp("/home/solderfumes/personal/img-viewer-rust/1bpp-320x240.bmp")?;
    let image = ImageView::new(ImageInfo::mono8(320, 240), &data);

    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
      if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    Ok(())
}
//fn main() {
//    let res = read_bmp::read_bmp("/home/solderfumes/personal/img-viewer-rust/1bpp-320x240.bmp");
//    if res.is_err() {
//        println!("What happpened??");
//        let _ = dbg!(res);
//    }
//
//    open_window::open_window();
//
//}
