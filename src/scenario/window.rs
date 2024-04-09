use xcap::Window;

#[cfg(feature = "libraries")]
fn window_capture() {
    let windows = Window::all().unwrap();
    for window in windows {
        let image = window.capture_image().unwrap();
        image
            .save(format!("window-{}.png", window.title()))
            .unwrap();
    }
}

#[cfg(not(feature = "libraries"))]
fn window_capture(){
    println!("La fonctionnalité de capture d'écran n'est pas prise en charge sur ce système");
}