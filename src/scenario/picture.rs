extern crate camera_capture;
extern crate image;

#[cfg(feature = "camera_linux")]
use std::io::Write;

#[cfg(feature = "camera_windows")]
use std::fs::File;
#[cfg(feature = "camera_windows")]
use std::path::Path;

#[cfg(target_os = "linux")]
#[cfg(feature = "camera_linux")]
fn picture(){
    let mut camera = rscam::new("/dev/video0").unwrap();
    camera.start(&rscam::Config {
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    }).unwrap();
    let frame = camera.capture().unwrap();
    camera.stop().unwrap();
    let mut file = std::fs::File::create("photo.jpg").unwrap();
    file.write_all(&frame).unwrap();
}

#[cfg(target_os = "windows")]
#[cfg(feature = "camera_windows")]
pub(crate) fn picture() {
    let cam = camera_capture::create(0).unwrap();

    let mut cam_iter = cam.fps(5.0).unwrap().start().unwrap();
    let img = cam_iter.next().unwrap();

    let file_name = "test.png";
    let path = Path::new(&file_name);
    let _ = &mut File::create(&path).unwrap();
    img.save(&path).unwrap();

    println!("img saved to {}", file_name);

}

#[cfg(not(feature = "camera_linux"))]
#[cfg(not(feature = "camera_windows"))]
fn picture(){
    println!("La fonctionnalité de capture d'image n'est pas prise en charge sur ce système");
}