mod connexion;
mod scan;
mod function_utils;

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;


//fn main() -> io::Result<()> {
//    scan::scan();
//
//    let connexion:TcpStream = connexion::co()?;
//    let connexion2:TcpStream = connexion.try_clone()?;
//
//    let thread_emission = thread::spawn(move|| {
//        connexion::emission(connexion);
//    });
//
//    let thread_reception = thread::spawn(move|| {
//        connexion::reception(connexion2);
//    });
//
//    thread_emission.join().expect("Thread emission erreur");
//    thread_reception.join().expect("Thread reception erreur");
//
//    Ok(())
//
//}

fn main() {
    scan::scan(scan::find_net())
    //scan::find_net();
    //println!("{:?}", scan::find_net());
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erreur lors de la saisis du message");

    input.trim().to_string()
}