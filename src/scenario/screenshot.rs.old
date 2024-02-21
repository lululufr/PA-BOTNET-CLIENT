use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

pub fn screenshot(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::primary()?;
    let (width, height) = (display.width(), display.height());
    let mut capturer = Capturer::new(display)?;

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let buffer = frame.to_vec();
                // Enregistrez l'image
                image::save_buffer(filename, &buffer, width as u32, height as u32, image::ColorType::Rgba8)?;
                break;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Attendre un peu avant de réessayer
                    thread::sleep(Duration::from_millis(100));
                    continue;
                } else {
                    return Err(Box::new(error));
                }
            }
        }
    }

    Ok(())
}