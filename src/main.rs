mod connexion;

mod function_utils;
mod scenario;

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;


fn main() -> io::Result<()> {


    let connexion:TcpStream = connexion::co()?;
    let connexion2:TcpStream = connexion.try_clone()?;

    let thread_emission = thread::spawn(move|| {
        connexion::emission(connexion);
    });

    let thread_reception = thread::spawn(move|| {
        connexion::reception(connexion2);
    });


    thread_reception.join().expect("Thread reception erreur");
    thread_emission.join().expect("Thread emission erreur");


    //switch case



    Ok(())

}

//fn main() {
//    //scan::scan(scan::find_net())
//    scan::find_net();
//    //println!("{:?}", scan::find_net());
//}

