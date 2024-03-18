#[cfg(feature = "libraries_windows")]
use scrap::{Capturer, Display};
#[cfg(feature = "libraries_windows")]
use std::io::ErrorKind::WouldBlock;
#[cfg(feature = "libraries_windows")]
use std::thread;
#[cfg(feature = "libraries_windows")]
use std::time::Duration;


#[cfg(target_os = "linux")]
#[cfg(feature = "libraries")]
fn monitor_capture() -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    for monitor in monitors {
        let image = monitor.capture_image().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        image
            .save(format!("monitor-{}.png", monitor.name()))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
#[cfg(feature = "libraries_windows")]
fn monitor_capture() -> Result<(), Box<dyn std::error::Error>> {
    // Définition d'un nom de fichier par défaut
    let filename = "capture.png";

    let display = Display::primary()?;
    let (width, height) = (display.width(), display.height());
    let mut capturer = Capturer::new(display)?;

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let buffer = frame.to_vec();
                // Enregistrez l'image avec le nom de fichier par défaut
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

#[cfg(not(feature = "libraries"))]
#[cfg(not(feature = "libraries_windows"))]
fn monitor_capture(){
    println!("La fonctionnalité de capture d'écran n'est pas prise en charge sur ce système");
}